# GitHub discussions for organization repository

Provide GitHub discussions for an organization repository

## Verify

https://github.com/orgs/[slug]/discussions must contain "View all discussions" (matched case-insensitively)

https://github.com/[slug]/[slug]/discussions must redirect to https://github.com/orgs/[slug]/discussions

## First posts

Each default category (Announcements, General, Ideas, Polls, Q&A, Show and tell) has a first post, written with the
`github-discussion-first-post-for-<category>` skill, in `locales/en-us/communications/github-discussions/<category>.txt`
(`qa` and `show-and-tell` for the last two): the title on the first line, a blank line, then the Markdown body, to paste
into GitHub. Every claim in a post comes from this repository's own documents.

`bin/test-github-discussions` checks both links above and that each post has a title, a blank line and a body.
