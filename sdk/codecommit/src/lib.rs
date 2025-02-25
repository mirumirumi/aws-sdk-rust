#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS CodeCommit</fullname>
//! <p>This is the <i>AWS CodeCommit API Reference</i>. This reference provides descriptions of the operations and data types for
//! AWS CodeCommit API along with usage examples.</p>
//! <p>You can use the AWS CodeCommit API to work with the following objects:</p>
//!
//! <p>Repositories, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>BatchGetRepositories</a>, which returns information about one or more repositories associated with your AWS account.</p>
//! </li>
//! <li>
//! <p>
//! <a>CreateRepository</a>, which creates an AWS CodeCommit repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeleteRepository</a>, which deletes an AWS CodeCommit repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetRepository</a>, which returns information about a specified repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListRepositories</a>, which lists all AWS CodeCommit repositories associated with your AWS account.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateRepositoryDescription</a>, which sets or updates the description of the repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateRepositoryName</a>, which changes the name of the
//! repository. If you change the name of a repository, no other users of that
//! repository can access it until you send them the new HTTPS or SSH URL to
//! use.</p>
//! </li>
//! </ul>
//!
//! <p>Branches, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>CreateBranch</a>, which creates a branch in a specified
//! repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeleteBranch</a>, which deletes the specified branch in a repository unless it is the default branch.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetBranch</a>, which returns information about a specified branch.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListBranches</a>, which lists all branches for a specified repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateDefaultBranch</a>, which changes the default branch for a repository.</p>
//! </li>
//! </ul>
//!
//! <p>Files, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>DeleteFile</a>, which deletes the content of a specified file from a specified branch.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetBlob</a>, which returns the base-64 encoded content of an
//! individual Git blob object in a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetFile</a>, which returns the base-64 encoded content of a specified file.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetFolder</a>, which returns the contents of a specified folder or directory.</p>
//! </li>
//! <li>
//! <p>
//! <a>PutFile</a>, which adds or modifies a single file in a specified repository and branch.</p>
//! </li>
//! </ul>
//!
//!
//! <p>Commits, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>BatchGetCommits</a>, which returns information about one or more commits in a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>CreateCommit</a>, which creates a commit for changes to a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetCommit</a>, which returns information about  a commit, including commit
//! messages and author and committer information.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetDifferences</a>, which returns information about the
//! differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID,
//! or other fully qualified reference).</p>
//! </li>
//! </ul>
//!
//!
//! <p>Merges, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>BatchDescribeMergeConflicts</a>, which returns information about conflicts in a merge between commits in a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>CreateUnreferencedMergeCommit</a>, which creates an unreferenced commit between two branches or commits for the
//! purpose of comparing them and identifying any potential conflicts.</p>
//! </li>
//! <li>
//! <p>
//! <a>DescribeMergeConflicts</a>, which returns information about merge conflicts between the base, source, and destination versions
//! of a file in a potential merge.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetMergeCommit</a>, which returns information about the merge between a source and destination commit. </p>
//!
//! </li>
//! <li>
//! <p>
//! <a>GetMergeConflicts</a>, which returns information about merge conflicts
//! between the source and destination branch in a pull request.</p>
//! </li>
//! <li>
//!
//! <p>
//! <a>GetMergeOptions</a>, which returns information about the available merge options between two branches or commit specifiers.</p>
//! </li>
//! <li>
//! <p>
//! <a>MergeBranchesByFastForward</a>, which merges two branches using the fast-forward merge option.</p>
//! </li>
//! <li>
//! <p>
//! <a>MergeBranchesBySquash</a>, which merges two branches using the squash merge option.</p>
//! </li>
//! <li>
//! <p>
//! <a>MergeBranchesByThreeWay</a>, which merges two branches using the three-way merge option.</p>
//! </li>
//! </ul>
//!
//!
//! <p>Pull requests, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>CreatePullRequest</a>, which creates a pull request in a specified repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>CreatePullRequestApprovalRule</a>, which creates an approval rule for a specified pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeletePullRequestApprovalRule</a>, which deletes an approval rule for a specified pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>DescribePullRequestEvents</a>, which returns information about one or more pull request events.</p>
//! </li>
//! <li>
//! <p>
//! <a>EvaluatePullRequestApprovalRules</a>, which evaluates whether a pull request has met all the conditions specified in its associated approval rules.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetCommentsForPullRequest</a>, which returns information about comments on a specified pull request.</p>
//! </li>
//! <li>
//!
//! <p>
//! <a>GetPullRequest</a>, which returns information about a specified pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetPullRequestApprovalStates</a>, which returns information about the approval states for a specified pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetPullRequestOverrideState</a>, which returns information about whether approval rules have been set aside (overriden) for a
//! pull request, and if so, the Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListPullRequests</a>, which lists all pull requests for a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>MergePullRequestByFastForward</a>, which merges the source destination branch of a pull request into the specified destination
//! branch for that pull request using the fast-forward merge option.</p>
//! </li>
//! <li>
//! <p>
//! <a>MergePullRequestBySquash</a>, which merges the source destination branch of a pull request into the specified destination
//! branch for that pull request using the squash merge option.</p>
//! </li>
//! <li>
//! <p>
//! <a>MergePullRequestByThreeWay</a>. which merges the source destination branch of a pull request into the specified destination
//! branch for that pull request using the three-way merge option.</p>
//! </li>
//! <li>
//! <p>
//! <a>OverridePullRequestApprovalRules</a>, which sets aside all approval rule requirements for a pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>PostCommentForPullRequest</a>, which posts a comment to a pull request at the specified line, file, or request.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdatePullRequestApprovalRuleContent</a>, which updates the structure of an approval rule for a pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdatePullRequestApprovalState</a>, which updates the state of an approval on a pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdatePullRequestDescription</a>, which updates the description of a pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdatePullRequestStatus</a>, which updates the status of a pull request.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdatePullRequestTitle</a>, which updates the title of a pull request.</p>
//! </li>
//! </ul>
//!
//! <p>Approval rule templates, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>AssociateApprovalRuleTemplateWithRepository</a>, which associates a template
//! with a specified repository. After the template is associated with a repository,
//! AWS CodeCommit creates approval rules that match the template conditions on
//! every pull request created in the specified repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>BatchAssociateApprovalRuleTemplateWithRepositories</a>, which associates a
//! template with one or more specified repositories. After the template is
//! associated with a repository, AWS CodeCommit creates approval rules that match
//! the template conditions on every pull request created in the specified
//! repositories.</p>
//! </li>
//! <li>
//! <p>
//! <a>BatchDisassociateApprovalRuleTemplateFromRepositories</a>, which removes the
//! association between a template and specified repositories so that approval rules
//! based on the template are not automatically created when pull requests are
//! created in those repositories.</p>
//! </li>
//! <li>
//! <p>
//! <a>CreateApprovalRuleTemplate</a>, which creates a template for approval rules that can then be associated with one or more repositories
//! in your AWS account.</p>
//! </li>
//! <li>
//! <p>
//! <a>DeleteApprovalRuleTemplate</a>, which deletes the specified template. It does not remove approval rules on pull requests already created with the template.</p>
//! </li>
//! <li>
//! <p>
//! <a>DisassociateApprovalRuleTemplateFromRepository</a>, which removes the
//! association between a template and a repository so that approval rules based on
//! the template are not automatically created when pull requests are created in the
//! specified repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetApprovalRuleTemplate</a>, which returns information about an approval rule template.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListApprovalRuleTemplates</a>, which lists all approval rule templates in the AWS Region in your AWS account.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListAssociatedApprovalRuleTemplatesForRepository</a>, which lists all approval rule templates that are associated with a specified repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>ListRepositoriesForApprovalRuleTemplate</a>, which lists all repositories associated with the specified approval rule template.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateApprovalRuleTemplateDescription</a>, which updates the description of an
//! approval rule template.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateApprovalRuleTemplateName</a>, which updates the name of an approval rule template.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateApprovalRuleTemplateContent</a>, which updates the content of an approval rule template.</p>
//! </li>
//! </ul>
//!
//! <p>Comments in a repository, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>DeleteCommentContent</a>, which deletes the content of a comment on a commit in a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetComment</a>, which returns information about a comment on a commit.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetCommentReactions</a>, which returns information about emoji reactions to comments.</p>
//! </li>
//! <li>
//! <p>
//! <a>GetCommentsForComparedCommit</a>, which returns information about comments on the comparison between two commit specifiers
//! in a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>PostCommentForComparedCommit</a>, which creates a comment on the comparison between two commit specifiers in a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>PostCommentReply</a>, which creates a reply to a comment.</p>
//! </li>
//! <li>
//! <p>
//! <a>PutCommentReaction</a>, which creates or updates an emoji reaction to a comment.</p>
//! </li>
//! <li>
//! <p>
//! <a>UpdateComment</a>, which updates the content of a comment on a commit in a repository.</p>
//! </li>
//! </ul>
//!
//! <p>Tags used to tag resources in AWS CodeCommit (not Git tags), by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>ListTagsForResource</a>, which gets information about AWS tags for a specified Amazon Resource Name (ARN) in AWS CodeCommit.</p>
//! </li>
//! <li>
//! <p>
//! <a>TagResource</a>, which adds or updates tags for a resource in AWS CodeCommit.</p>
//! </li>
//! <li>
//! <p>
//! <a>UntagResource</a>, which removes tags for a resource in AWS CodeCommit.</p>
//! </li>
//! </ul>
//!
//! <p>Triggers, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <a>GetRepositoryTriggers</a>, which returns information about triggers configured
//! for a repository.</p>
//! </li>
//! <li>
//! <p>
//! <a>PutRepositoryTriggers</a>, which replaces all triggers for a repository and can
//! be used to create or delete triggers.</p>
//! </li>
//! <li>
//! <p>
//! <a>TestRepositoryTriggers</a>, which tests the functionality of a repository trigger
//! by sending data to the trigger target.</p>
//! </li>
//! </ul>
//!
//!
//!
//!
//! <p>For information about how to use AWS CodeCommit, see the <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">AWS CodeCommit User Guide</a>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

mod aws_endpoint;

mod idempotency_token;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("codecommit", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
