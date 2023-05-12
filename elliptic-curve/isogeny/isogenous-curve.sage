#!/usr/bin/sage
# vim: syntax=python

# look for isogenous curves having j-invariant not in {0, 1728}
def find_iso(E):
    for x in primes(30): 
        iso =[i for i in  E.isogenies_prime_degree(x)
              if i.codomain().j_invariant() not in (0, 1728)]
        if len(iso) > 0:
            return iso
    return None    

# secp256k1
p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
E = EllipticCurve(GF(p), [0,7])
iso = find_iso(E);iso