#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "codex-rs/hepta-infer-core/src/private_protocol.rs"

text = TARGET.read_text(encoding="utf-8")
text = text.replace("use crate::AcceptedEvent;\n", "", 1)
text = text.replace(
    "        assert_eq!(\n            DaemonToWorker::decode_canonical(&encoded),\n            Err(InferError::ProtocolShape)\n        );\n",
    "        assert!(matches!(\n            DaemonToWorker::decode_canonical(&encoded),\n            Err(InferError::ProtocolShape)\n        ));\n",
    1,
)
text = text.replace(
    "        assert_eq!(\n            WorkerToDaemon::decode_canonical(&encoded),\n            Err(InferError::ProtocolTrailingData)\n        );\n",
    "        assert!(matches!(\n            WorkerToDaemon::decode_canonical(&encoded),\n            Err(InferError::ProtocolTrailingData)\n        ));\n",
    1,
)
text = text.replace(
    '''\n        let accepted = AcceptedEvent {\n            request_id: must(RequestId::parse("request-unused")),\n            request_generation: 1,\n            backend_generation: 2,\n            sequence: 1,\n        };\n        assert_eq!(accepted.sequence, 1);\n''',
    "\n",
    1,
)
TARGET.write_text(text, encoding="utf-8")
