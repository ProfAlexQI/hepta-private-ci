# frozen_string_literal: true

require "digest"
require "json"
require "open3"

module HeptaMigrationHistoryAnchorV1
  module_function

  def verify!(root:, anchor_path:, receipt_path:)
    anchor = JSON.parse(File.binread(anchor_path))
    raise "migration history anchor is not bound" unless anchor.fetch("status") == "history_bound" &&
      anchor.fetch("git_history_recovery_bound") && !anchor.fetch("history_anchor_required")

    result_commit = anchor.fetch("result_commit")
    result_tree = anchor.fetch("result_tree")
    actual_tree, tree_status = Open3.capture2(
      "git", "-C", root, "show", "-s", "--format=%T", result_commit
    )
    raise "migration history result commit is unavailable" unless tree_status.success?
    raise "migration history result tree drifted" unless actual_tree.strip == result_tree

    _ancestor_output, ancestor_status = Open3.capture2e(
      "git", "-C", root, "merge-base", "--is-ancestor", result_commit, "HEAD"
    )
    raise "current HEAD is not descended from the migration result commit" unless ancestor_status.success?

    relative_receipt_path = receipt_path.delete_prefix("#{root}/")
    receipt_record = anchor.fetch("receipts").find do |record|
      record.fetch("path") == relative_receipt_path
    end
    raise "migration history anchor omits receipt #{relative_receipt_path}" unless receipt_record

    receipt_sha256 = Digest::SHA256.file(receipt_path).hexdigest
    raise "migration history receipt hash drifted" unless receipt_record.fetch("sha256") == receipt_sha256

    {
      result_commit: result_commit,
      result_tree: result_tree,
      receipt_sha256: receipt_sha256,
      anchor_sha256: Digest::SHA256.file(anchor_path).hexdigest
    }
  end
end
