# Usher Requests

## Usher Signature

Upon receiving the request, if it does not contain an usher signature, we check the usher_public_key to see if that is us.

If it is, we validate the Rhex, ensuring all data is complete and the author signature is valid. After validation, we add Context (at, spacial_ref, spacial_data) and sign H(Context || AuthorSignature)

If not, then we reject the request.

### What the fuck that actually looks like

- lattice.usher.signature.request -> transform.lattice.usher.signature.issuer -> lattice.scope.cache.lookup + lattice.usher.signature.action
- lattice.scope.cache.result + lattice.usher.signature.action -> transform.lattice.usher.signature.resolver -> lattice.usher.signature.result

## Quorum Signature

Upon receiving the request, if it does have an usher signature, we check the scope to see if it is one we are a quorum member for.

If it is, we validate the Rhex, checking both signatures, ensuring all data is complete, we are within scope policy, and making sure the usher_public_key is a quorum member. Quorum signs over H(AuthorSignature || UsherSignature)

If not, then we reject the request.

## Final Submission

Upon receiving the request, if the current_hash is set, we check to make sure we meet K of N quorum signatures for the scope quorum set in the policy.

If so, we validate everything a final time, data, Context, signatures, policy adherance, etc. We then check to make sure we are a "append" usher for the specified scope. If anything fails, we reject.
