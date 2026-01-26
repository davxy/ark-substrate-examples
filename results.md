# pallet_ark_groth16

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_377_groth16_verify | 17.51 ms | 2.83 ms | 6.19x |
| bls12_381_groth16_verify | 16.61 ms | 2.83 ms | 5.88x |
| bw6_761_groth16_verify | 76.45 ms | 9.59 ms | 7.97x |

# pallet_ark_hostcalls

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_381_msm_g1_x_10 | 10.28 ms | 3.09 ms | 3.33x |
| bls12_381_msm_g1_x_100 | 70.56 ms | 25.46 ms | 2.77x |
| bls12_381_msm_g2_x_10 | 24.73 ms | 6.99 ms | 3.54x |
| bls12_381_msm_g2_x_100 | 167.78 ms | 61.67 ms | 2.72x |
| bls12_381_mul_affine_g1 | 1.46 ms | 1.46 ms | 1.00x |
| bls12_381_mul_affine_g2 | 3.76 ms | 3.76 ms | 1.00x |
| bls12_381_mul_projective_g1 | 739.66 us | 112.19 us | 6.59x |
| bls12_381_mul_projective_g2 | 3.38 ms | 512.46 us | 6.59x |
| bls12_381_pairing | 7.97 ms | 2.00 ms | 3.99x |
| ed_on_bls12_377_msm_te_x_10 | 8.94 ms | 2.31 ms | 3.88x |
| ed_on_bls12_377_msm_te_x_100 | 68.99 ms | 19.37 ms | 3.56x |
| ed_on_bls12_377_mul_affine_te | 1.13 ms | 273.59 us | 4.13x |
| ed_on_bls12_377_mul_projective_te | 489.66 us | 90.32 us | 5.42x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_10 | 8.05 ms | 2.83 ms | 2.84x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_100 | 67.64 ms | 20.31 ms | 3.33x |
| ed_on_bls12_381_bandersnatch_msm_te_x_10 | 8.74 ms | 2.20 ms | 3.98x |
| ed_on_bls12_381_bandersnatch_msm_te_x_100 | 66.71 ms | 19.04 ms | 3.50x |
| ed_on_bls12_381_bandersnatch_mul_affine_sw | 1.12 ms | 311.59 us | 3.58x |
| ed_on_bls12_381_bandersnatch_mul_affine_te | 1.06 ms | 258.24 us | 4.12x |
| ed_on_bls12_381_bandersnatch_mul_projective_sw | 649.22 us | 126.31 us | 5.14x |
| ed_on_bls12_381_bandersnatch_mul_projective_te | 510.03 us | 95.70 us | 5.33x |

# pallet_ark_vrf

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| ietf_vrf_verify | 2.10 ms | 599.48 us | 3.50x |
| ring_vrf_commit_buffered_x_1 | 3.53 ms | 1.45 ms | 2.43x |
| ring_vrf_commit_buffered_x_42 | 51.53 ms | 15.71 ms | 3.28x |
| ring_vrf_commit_unbuffered_x_1 | 26.06 us | 37.48 us | 0.70x |
| ring_vrf_commit_unbuffered_x_42 | 23.11 us | 38.40 us | 0.60x |
| ring_vrf_verify_x_1 | 30.89 ms | 16.83 ms | 1.84x |
| ring_vrf_verify_x_42 | 29.60 ms | 16.88 ms | 1.75x |
