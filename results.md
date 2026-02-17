# pallet_ark_groth16

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_377_groth16_verify | 17.66 ms | 3.11 ms | 5.68x |
| bls12_381_groth16_verify | 16.89 ms | 2.86 ms | 5.91x |
| bw6_761_groth16_verify | 76.70 ms | 9.99 ms | 7.68x |

# pallet_ark_hostcalls

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_381_msm_g1_x_10 | 10.05 ms | 3.08 ms | 3.27x |
| bls12_381_msm_g1_x_32 | 26.93 ms | 8.84 ms | 3.05x |
| bls12_381_msm_g1_x_55 | 44.43 ms | 13.65 ms | 3.26x |
| bls12_381_msm_g1_x_77 | 58.58 ms | 20.26 ms | 2.89x |
| bls12_381_msm_g1_x_100 | 73.34 ms | 25.81 ms | 2.84x |
| bls12_381_msm_g2_x_10 | 25.63 ms | 7.88 ms | 3.25x |
| bls12_381_msm_g2_x_32 | 62.51 ms | 22.16 ms | 2.82x |
| bls12_381_msm_g2_x_55 | 101.72 ms | 37.17 ms | 2.74x |
| bls12_381_msm_g2_x_77 | 145.59 ms | 51.78 ms | 2.81x |
| bls12_381_msm_g2_x_100 | 164.95 ms | 63.61 ms | 2.59x |
| bls12_381_mul_affine_g1 | 1.61 ms | 1.60 ms | 1.00x |
| bls12_381_mul_affine_g2 | 3.78 ms | 3.77 ms | 1.00x |
| bls12_381_mul_projective_g1 | 845.98 us | 119.68 us | 7.07x |
| bls12_381_mul_projective_g2 | 3.39 ms | 506.80 us | 6.69x |
| bls12_381_pairing | 7.88 ms | 1.94 ms | 4.05x |
| ed_on_bls12_377_msm_te_x_10 | 9.03 ms | 2.60 ms | 3.48x |
| ed_on_bls12_377_msm_te_x_32 | 24.90 ms | 6.79 ms | 3.66x |
| ed_on_bls12_377_msm_te_x_55 | 45.54 ms | 11.40 ms | 3.99x |
| ed_on_bls12_377_msm_te_x_77 | 60.32 ms | 15.81 ms | 3.82x |
| ed_on_bls12_377_msm_te_x_100 | 75.53 ms | 20.27 ms | 3.73x |
| ed_on_bls12_377_mul_affine_te | 1.11 ms | 302.07 us | 3.68x |
| ed_on_bls12_377_mul_projective_te | 493.13 us | 86.13 us | 5.73x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_10 | 9.14 ms | 2.54 ms | 3.60x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_32 | 26.52 ms | 7.35 ms | 3.61x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_55 | 44.22 ms | 11.67 ms | 3.79x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_77 | 60.32 ms | 16.15 ms | 3.74x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_100 | 76.90 ms | 20.90 ms | 3.68x |
| ed_on_bls12_381_bandersnatch_msm_te_x_10 | 9.55 ms | 2.11 ms | 4.54x |
| ed_on_bls12_381_bandersnatch_msm_te_x_32 | 26.48 ms | 5.86 ms | 4.52x |
| ed_on_bls12_381_bandersnatch_msm_te_x_55 | 40.88 ms | 9.69 ms | 4.22x |
| ed_on_bls12_381_bandersnatch_msm_te_x_77 | 53.37 ms | 14.08 ms | 3.79x |
| ed_on_bls12_381_bandersnatch_msm_te_x_100 | 66.68 ms | 17.18 ms | 3.88x |
| ed_on_bls12_381_bandersnatch_mul_affine_sw | 1.24 ms | 331.13 us | 3.75x |
| ed_on_bls12_381_bandersnatch_mul_affine_te | 969.43 us | 241.48 us | 4.01x |
| ed_on_bls12_381_bandersnatch_mul_projective_sw | 659.51 us | 125.91 us | 5.24x |
| ed_on_bls12_381_bandersnatch_mul_projective_te | 456.86 us | 85.76 us | 5.33x |

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
