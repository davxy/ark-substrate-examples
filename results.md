# pallet_ark_groth16

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_377_groth16_verify | 17.64 ms | 3.24 ms | 5.44x |
| bls12_381_groth16_verify | 16.67 ms | 2.89 ms | 5.77x |
| bw6_761_groth16_verify | 86.70 ms | 10.69 ms | 8.11x |

# pallet_ark_hostcalls

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_381_msm_g1_x_10 | 9.69 ms | 3.12 ms | 3.11x |
| bls12_381_msm_g1_x_32 | 26.50 ms | 8.83 ms | 3.00x |
| bls12_381_msm_g1_x_55 | 45.03 ms | 13.42 ms | 3.35x |
| bls12_381_msm_g1_x_77 | 56.32 ms | 20.05 ms | 2.81x |
| bls12_381_msm_g1_x_100 | 70.81 ms | 26.08 ms | 2.72x |
| bls12_381_msm_g2_x_10 | 25.31 ms | 7.10 ms | 3.56x |
| bls12_381_msm_g2_x_32 | 62.51 ms | 20.75 ms | 3.01x |
| bls12_381_msm_g2_x_55 | 101.78 ms | 34.71 ms | 2.93x |
| bls12_381_msm_g2_x_77 | 132.93 ms | 50.96 ms | 2.61x |
| bls12_381_msm_g2_x_100 | 165.95 ms | 63.46 ms | 2.62x |
| bls12_381_mul_affine_g1 | 1.56 ms | 1.55 ms | 1.01x |
| bls12_381_mul_affine_g2 | 3.78 ms | 3.78 ms | 1.00x |
| bls12_381_mul_projective_g1 | 786.06 us | 121.69 us | 6.46x |
| bls12_381_mul_projective_g2 | 3.39 ms | 511.39 us | 6.62x |
| bls12_381_pairing | 7.55 ms | 1.88 ms | 4.01x |
| ed_on_bls12_377_msm_te_x_10 | 8.99 ms | 2.44 ms | 3.68x |
| ed_on_bls12_377_msm_te_x_32 | 24.87 ms | 6.32 ms | 3.93x |
| ed_on_bls12_377_msm_te_x_55 | 41.57 ms | 11.87 ms | 3.50x |
| ed_on_bls12_377_msm_te_x_77 | 55.09 ms | 16.52 ms | 3.34x |
| ed_on_bls12_377_msm_te_x_100 | 69.79 ms | 20.41 ms | 3.42x |
| ed_on_bls12_377_mul_affine_te | 1.11 ms | 302.66 us | 3.68x |
| ed_on_bls12_377_mul_projective_te | 487.42 us | 93.99 us | 5.19x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_10 | 8.93 ms | 2.49 ms | 3.59x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_32 | 26.71 ms | 6.47 ms | 4.13x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_55 | 42.85 ms | 11.53 ms | 3.72x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_77 | 58.58 ms | 16.71 ms | 3.51x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_100 | 68.35 ms | 21.19 ms | 3.22x |
| ed_on_bls12_381_bandersnatch_msm_te_x_10 | 9.57 ms | 2.31 ms | 4.14x |
| ed_on_bls12_381_bandersnatch_msm_te_x_32 | 26.60 ms | 6.25 ms | 4.25x |
| ed_on_bls12_381_bandersnatch_msm_te_x_55 | 44.51 ms | 10.39 ms | 4.28x |
| ed_on_bls12_381_bandersnatch_msm_te_x_77 | 58.35 ms | 14.82 ms | 3.94x |
| ed_on_bls12_381_bandersnatch_msm_te_x_100 | 72.39 ms | 18.73 ms | 3.86x |
| ed_on_bls12_381_bandersnatch_mul_affine_sw | 1.20 ms | 332.05 us | 3.60x |
| ed_on_bls12_381_bandersnatch_mul_affine_te | 1.06 ms | 264.05 us | 4.02x |
| ed_on_bls12_381_bandersnatch_mul_projective_sw | 661.88 us | 126.76 us | 5.22x |
| ed_on_bls12_381_bandersnatch_mul_projective_te | 504.71 us | 94.56 us | 5.34x |

# pallet_ark_vrf

Storing SRS items uncompressed (96 bytes instead of 48) eliminates the point
decompression cost (field square root) on every `fetch_srs_range` call.
The accumulate operations (push of new members during ring commitment
construction), which are the main consumers of SRS reads, show a
~2.5-3x speedup in the substrate column at scale (e.g. x_50: 18.95 ms -> 6.44 ms).
The arkworks column also benefits (~35-40%) as both paths read SRS from storage.
Verify and IETF operations are unaffected (no SRS reads).

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| ietf_vrf_verify | 2.14 ms | 621.79 us | 3.44x |
| ring_vrf_accumulate_and_commit_x_1 | 3.40 ms | 1.45 ms | 2.35x |
| ring_vrf_accumulate_and_commit_x_13 | 13.03 ms | 2.60 ms | 5.01x |
| ring_vrf_accumulate_and_commit_x_25 | 20.66 ms | 3.86 ms | 5.36x |
| ring_vrf_accumulate_and_commit_x_37 | 32.31 ms | 5.25 ms | 6.16x |
| ring_vrf_accumulate_and_commit_x_50 | 37.69 ms | 6.74 ms | 5.59x |
| ring_vrf_accumulate_x_1 | 3.49 ms | 1.45 ms | 2.40x |
| ring_vrf_accumulate_x_13 | 11.91 ms | 2.86 ms | 4.16x |
| ring_vrf_accumulate_x_25 | 18.94 ms | 3.98 ms | 4.75x |
| ring_vrf_accumulate_x_37 | 29.56 ms | 5.60 ms | 5.28x |
| ring_vrf_accumulate_x_50 | 34.80 ms | 6.44 ms | 5.41x |
| ring_vrf_commit | 23.71 us | 47.44 us | 0.50x |
| ring_vrf_verify | 31.05 ms | 17.61 ms | 1.76x |
| ring_vrf_verify_batch_x_1 | 31.09 ms | 19.06 ms | 1.63x |
| ring_vrf_verify_batch_x_4 | 97.26 ms | 49.82 ms | 1.95x |
| ring_vrf_verify_batch_x_8 | 185.58 ms | 100.66 ms | 1.84x |
| ring_vrf_verify_batch_x_12 | 274.60 ms | 145.68 ms | 1.89x |
| ring_vrf_verify_batch_x_16 | 363.28 ms | 180.66 ms | 2.01x |
