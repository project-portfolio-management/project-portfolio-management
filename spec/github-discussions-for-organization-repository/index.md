# GitHub discussions for organization repository

Provide GitHub discussions for an organization repository

## Verify

https://github.com/orgs/[slug]/discussions must contain "View all discussions" (matched case-insensitively)

https://github.com/[slug]/[slug]/discussions must redirect to https://github.com/orgs/[slug]/discussions

`bin/test-github-discussions` checks both links, and the category descriptions in
`locales/en-us/communications/github-discussions/*.txt`.
