use std::{fmt::Display, io};

use codex_keyring_store::{CredentialStoreError, KeyringStore};
use jni::{
    JNIEnv,
    errors::Result as JniResult,
    objects::{JByteArray, JObject, JObjectArray, JString, JValue},
};

use super::android_contract::{
    AndroidCredentialIdentity, credential_identity, decode_protected_blob, encode_protected_blob,
};

const PREFERENCES_NAME: &str = "hepta_secure_matrix_credentials_v1";
const KEYSTORE_PROVIDER: &str = "AndroidKeyStore";
const AES_ALGORITHM: &str = "AES";
const AES_GCM_CIPHER: &str = "AES/GCM/NoPadding";

#[derive(Debug)]
pub(super) struct AndroidKeystoreStore;

fn credential_error(context: &'static str, error: impl Display) -> CredentialStoreError {
    CredentialStoreError::new(keyring::Error::PlatformFailure(Box::new(io::Error::other(
        format!("{context}: {error}"),
    ))))
}

fn clear_exception(env: &mut JNIEnv<'_>) {
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_clear();
    }
}

fn with_android<T, F>(context: &'static str, operation: F) -> Result<T, CredentialStoreError>
where
    F: for<'a, 'b, 'c, 'd> FnOnce(&'a mut JNIEnv<'b>, &'c JObject<'d>) -> JniResult<T>,
{
    let result = robius_android_env::with_activity(|env, activity| {
        let result = operation(env, activity);
        if result.is_err() {
            clear_exception(env);
        }
        result
    })
    .map_err(|error| credential_error(context, error))?;
    result.map_err(|error| credential_error(context, error))
}

fn key_store<'local>(env: &mut JNIEnv<'local>) -> JniResult<JObject<'local>> {
    let provider = JObject::from(env.new_string(KEYSTORE_PROVIDER)?);
    let store = env
        .call_static_method(
            "java/security/KeyStore",
            "getInstance",
            "(Ljava/lang/String;)Ljava/security/KeyStore;",
            &[JValue::Object(&provider)],
        )?
        .l()?;
    let null = JObject::null();
    env.call_method(
        &store,
        "load",
        "(Ljava/security/KeyStore$LoadStoreParameter;)V",
        &[JValue::Object(&null)],
    )?
    .v()?;
    Ok(store)
}

fn alias_exists(env: &mut JNIEnv<'_>, store: &JObject<'_>, alias: &str) -> JniResult<bool> {
    let alias = JObject::from(env.new_string(alias)?);
    env.call_method(
        store,
        "containsAlias",
        "(Ljava/lang/String;)Z",
        &[JValue::Object(&alias)],
    )?
    .z()
}

fn string_array<'local>(
    env: &mut JNIEnv<'local>,
    values: &[&str],
) -> JniResult<JObjectArray<'local>> {
    let array = env.new_object_array(values.len() as i32, "java/lang/String", JObject::null())?;
    for (index, value) in values.iter().enumerate() {
        env.set_object_array_element(&array, index as i32, env.new_string(value)?)?;
    }
    Ok(array)
}

fn ensure_secret_key(env: &mut JNIEnv<'_>, store: &JObject<'_>, alias: &str) -> JniResult<bool> {
    if alias_exists(env, store, alias)? {
        return Ok(false);
    }
    let algorithm = JObject::from(env.new_string(AES_ALGORITHM)?);
    let provider = JObject::from(env.new_string(KEYSTORE_PROVIDER)?);
    let generator = env
        .call_static_method(
            "javax/crypto/KeyGenerator",
            "getInstance",
            "(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/KeyGenerator;",
            &[JValue::Object(&algorithm), JValue::Object(&provider)],
        )?
        .l()?;
    let alias = JObject::from(env.new_string(alias)?);
    let builder = env.new_object(
        "android/security/keystore/KeyGenParameterSpec$Builder",
        "(Ljava/lang/String;I)V",
        &[JValue::Object(&alias), JValue::Int(3)],
    )?;
    let modes = JObject::from(string_array(env, &["GCM"])?);
    env.call_method(
        &builder,
        "setBlockModes",
        "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
        &[JValue::Object(&modes)],
    )?;
    let paddings = JObject::from(string_array(env, &["NoPadding"])?);
    env.call_method(
        &builder,
        "setEncryptionPaddings",
        "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
        &[JValue::Object(&paddings)],
    )?;
    env.call_method(
        &builder,
        "setKeySize",
        "(I)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
        &[JValue::Int(256)],
    )?;
    env.call_method(
        &builder,
        "setRandomizedEncryptionRequired",
        "(Z)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
        &[JValue::Bool(1)],
    )?;
    let spec = env
        .call_method(
            &builder,
            "build",
            "()Landroid/security/keystore/KeyGenParameterSpec;",
            &[],
        )?
        .l()?;
    env.call_method(
        &generator,
        "init",
        "(Ljava/security/spec/AlgorithmParameterSpec;)V",
        &[JValue::Object(&spec)],
    )?;
    env.call_method(&generator, "generateKey", "()Ljavax/crypto/SecretKey;", &[])?;
    Ok(true)
}

fn secret_key<'local>(
    env: &mut JNIEnv<'local>,
    store: &JObject<'local>,
    alias: &str,
) -> JniResult<JObject<'local>> {
    let alias = JObject::from(env.new_string(alias)?);
    let null = JObject::null();
    let key = env
        .call_method(
            store,
            "getKey",
            "(Ljava/lang/String;[C)Ljava/security/Key;",
            &[JValue::Object(&alias), JValue::Object(&null)],
        )?
        .l()?;
    if key.is_null() {
        Err(jni::errors::Error::NullPtr("AndroidKeyStore secret key"))
    } else {
        Ok(key)
    }
}

fn cipher<'local>(env: &mut JNIEnv<'local>) -> JniResult<JObject<'local>> {
    let transformation = JObject::from(env.new_string(AES_GCM_CIPHER)?);
    env.call_static_method(
        "javax/crypto/Cipher",
        "getInstance",
        "(Ljava/lang/String;)Ljavax/crypto/Cipher;",
        &[JValue::Object(&transformation)],
    )?
    .l()
}

fn encrypt(
    env: &mut JNIEnv<'_>,
    key: &JObject<'_>,
    aad: &[u8],
    plaintext: &[u8],
) -> JniResult<(Vec<u8>, Vec<u8>)> {
    let cipher = cipher(env)?;
    env.call_method(
        &cipher,
        "init",
        "(ILjava/security/Key;)V",
        &[JValue::Int(1), JValue::Object(key)],
    )?;
    let aad = JObject::from(env.byte_array_from_slice(aad)?);
    env.call_method(&cipher, "updateAAD", "([B)V", &[JValue::Object(&aad)])?;
    let plaintext = JObject::from(env.byte_array_from_slice(plaintext)?);
    let encrypted = env
        .call_method(&cipher, "doFinal", "([B)[B", &[JValue::Object(&plaintext)])?
        .l()?;
    let iv = env.call_method(&cipher, "getIV", "()[B", &[])?.l()?;
    Ok((
        env.convert_byte_array(JByteArray::from(iv))?,
        env.convert_byte_array(JByteArray::from(encrypted))?,
    ))
}

fn decrypt(
    env: &mut JNIEnv<'_>,
    key: &JObject<'_>,
    aad: &[u8],
    iv: &[u8],
    ciphertext: &[u8],
) -> JniResult<Vec<u8>> {
    let cipher = cipher(env)?;
    let iv = JObject::from(env.byte_array_from_slice(iv)?);
    let spec = env.new_object(
        "javax/crypto/spec/GCMParameterSpec",
        "(I[B)V",
        &[JValue::Int(128), JValue::Object(&iv)],
    )?;
    env.call_method(
        &cipher,
        "init",
        "(ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V",
        &[JValue::Int(2), JValue::Object(key), JValue::Object(&spec)],
    )?;
    let aad = JObject::from(env.byte_array_from_slice(aad)?);
    env.call_method(&cipher, "updateAAD", "([B)V", &[JValue::Object(&aad)])?;
    let encrypted = JObject::from(env.byte_array_from_slice(ciphertext)?);
    let plaintext = env
        .call_method(&cipher, "doFinal", "([B)[B", &[JValue::Object(&encrypted)])?
        .l()?;
    env.convert_byte_array(JByteArray::from(plaintext))
}

fn preferences<'local>(
    env: &mut JNIEnv<'local>,
    activity: &JObject<'_>,
) -> JniResult<JObject<'local>> {
    let name = JObject::from(env.new_string(PREFERENCES_NAME)?);
    env.call_method(
        activity,
        "getSharedPreferences",
        "(Ljava/lang/String;I)Landroid/content/SharedPreferences;",
        &[JValue::Object(&name), JValue::Int(0)],
    )?
    .l()
}

fn read_preference(
    env: &mut JNIEnv<'_>,
    activity: &JObject<'_>,
    key: &str,
) -> JniResult<Option<String>> {
    let preferences = preferences(env, activity)?;
    let key = JObject::from(env.new_string(key)?);
    if !env
        .call_method(
            &preferences,
            "contains",
            "(Ljava/lang/String;)Z",
            &[JValue::Object(&key)],
        )?
        .z()?
    {
        return Ok(None);
    }
    let null = JObject::null();
    let value = env
        .call_method(
            &preferences,
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[JValue::Object(&key), JValue::Object(&null)],
        )?
        .l()?;
    if value.is_null() {
        return Err(jni::errors::Error::NullPtr("Android credential preference"));
    }
    Ok(Some(env.get_string(&JString::from(value))?.into()))
}

fn write_preference(
    env: &mut JNIEnv<'_>,
    activity: &JObject<'_>,
    key: &str,
    value: Option<&str>,
) -> JniResult<()> {
    let preferences = preferences(env, activity)?;
    let editor = env
        .call_method(
            &preferences,
            "edit",
            "()Landroid/content/SharedPreferences$Editor;",
            &[],
        )?
        .l()?;
    let key = JObject::from(env.new_string(key)?);
    if let Some(value) = value {
        let value = JObject::from(env.new_string(value)?);
        env.call_method(
            &editor,
            "putString",
            "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;",
            &[JValue::Object(&key), JValue::Object(&value)],
        )?;
    } else {
        env.call_method(
            &editor,
            "remove",
            "(Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;",
            &[JValue::Object(&key)],
        )?;
    }
    if env.call_method(&editor, "commit", "()Z", &[])?.z()? {
        Ok(())
    } else {
        Err(jni::errors::Error::JniCall(jni::errors::JniError::Unknown))
    }
}

fn delete_key(env: &mut JNIEnv<'_>, store: &JObject<'_>, alias: &str) -> JniResult<()> {
    if alias_exists(env, store, alias)? {
        let alias = JObject::from(env.new_string(alias)?);
        env.call_method(
            store,
            "deleteEntry",
            "(Ljava/lang/String;)V",
            &[JValue::Object(&alias)],
        )?;
    }
    Ok(())
}

impl KeyringStore for AndroidKeystoreStore {
    fn load(&self, service: &str, account: &str) -> Result<Option<String>, CredentialStoreError> {
        let identity = credential_identity(service, account)
            .map_err(|error| credential_error("invalid Android credential identity", error))?;
        let encoded = with_android(
            "failed to read Android credential envelope",
            |env, activity| read_preference(env, activity, &identity.preference_key),
        )?;
        let Some(encoded) = encoded else {
            return Ok(None);
        };
        let blob = decode_protected_blob(&encoded)
            .map_err(|error| credential_error("invalid Android credential envelope", error))?;
        let plaintext = with_android("failed to decrypt Android credential", |env, _| {
            let store = key_store(env)?;
            let key = secret_key(env, &store, &identity.key_alias)?;
            decrypt(env, &key, &identity.aad, &blob.iv, &blob.ciphertext)
        })?;
        String::from_utf8(plaintext).map(Some).map_err(|error| {
            CredentialStoreError::new(keyring::Error::BadEncoding(error.into_bytes()))
        })
    }

    fn save(&self, service: &str, account: &str, value: &str) -> Result<(), CredentialStoreError> {
        let identity = credential_identity(service, account)
            .map_err(|error| credential_error("invalid Android credential identity", error))?;
        let (generated, iv, ciphertext) =
            with_android("failed to encrypt Android credential", |env, _| {
                let store = key_store(env)?;
                let generated = ensure_secret_key(env, &store, &identity.key_alias)?;
                let key = secret_key(env, &store, &identity.key_alias)?;
                let (iv, ciphertext) = encrypt(env, &key, &identity.aad, value.as_bytes())?;
                Ok((generated, iv, ciphertext))
            })?;
        let encoded = encode_protected_blob(&iv, &ciphertext)
            .map_err(|error| credential_error("invalid Android encrypted credential", error))?;
        let write = with_android(
            "failed to persist Android credential envelope",
            |env, activity| {
                write_preference(env, activity, &identity.preference_key, Some(&encoded))
            },
        );
        if write.is_err() && generated {
            let _ = with_android("failed to roll back Android credential key", |env, _| {
                let store = key_store(env)?;
                delete_key(env, &store, &identity.key_alias)
            });
        }
        write
    }

    fn delete(&self, service: &str, account: &str) -> Result<bool, CredentialStoreError> {
        let identity: AndroidCredentialIdentity = credential_identity(service, account)
            .map_err(|error| credential_error("invalid Android credential identity", error))?;
        let existed = with_android(
            "failed to read Android credential envelope",
            |env, activity| Ok(read_preference(env, activity, &identity.preference_key)?.is_some()),
        )?;
        if existed {
            with_android(
                "failed to remove Android credential envelope",
                |env, activity| write_preference(env, activity, &identity.preference_key, None),
            )?;
        }
        with_android("failed to remove Android credential key", |env, _| {
            let store = key_store(env)?;
            delete_key(env, &store, &identity.key_alias)
        })?;
        Ok(existed)
    }
}
