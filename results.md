# pallet_ark_groth16

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_377_groth16_verify | 17.66 ms | 3.11 ms | 5.68x |
| bls12_381_groth16_verify | 16.89 ms | 2.86 ms | 5.91x |
| bw6_761_groth16_verify | 76.70 ms | 9.99 ms | 7.68x |

# pallet_ark_hostcalls

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_381_msm_g1_x_10 | 9.84 ms | 3.32 ms | 2.97x |
| bls12_381_msm_g1_x_100 | 75.54 ms | 26.90 ms | 2.81x |
| bls12_381_msm_g1_x_32 | 27.05 ms | 9.15 ms | 2.96x |
| bls12_381_msm_g1_x_55 | 44.20 ms | 15.12 ms | 2.92x |
| bls12_381_msm_g1_x_77 | 59.21 ms | 20.94 ms | 2.83x |
| bls12_381_msm_g2_x_10 | 23.48 ms | 7.50 ms | 3.13x |
| bls12_381_msm_g2_x_100 | 165.13 ms | 64.84 ms | 2.55x |
| bls12_381_msm_g2_x_32 | 62.51 ms | 21.46 ms | 2.91x |
| bls12_381_msm_g2_x_55 | 111.38 ms | 36.16 ms | 3.08x |
| bls12_381_msm_g2_x_77 | 137.84 ms | 50.32 ms | 2.74x |
| bls12_381_mul_affine_g1 | 1.47 ms | 400.38 us | 3.67x |
| bls12_381_mul_affine_g2 | 3.78 ms | 1.05 ms | 3.62x |
| bls12_381_mul_projective_g1 | 730.09 us | 171.84 us | 4.25x |
| bls12_381_mul_projective_g2 | 3.53 ms | 468.70 us | 7.52x |
| bls12_381_pairing | 7.50 ms | 1.86 ms | 4.04x |
| ed_on_bls12_377_msm_te_x_10 | 8.98 ms | 2.55 ms | 3.53x |
| ed_on_bls12_377_msm_te_x_100 | 69.04 ms | 20.37 ms | 3.39x |
| ed_on_bls12_377_msm_te_x_32 | 24.88 ms | 6.93 ms | 3.59x |
| ed_on_bls12_377_msm_te_x_55 | 41.64 ms | 11.53 ms | 3.61x |
| ed_on_bls12_377_msm_te_x_77 | 55.13 ms | 15.94 ms | 3.46x |
| ed_on_bls12_377_mul_affine_te | 1.02 ms | 289.90 us | 3.51x |
| ed_on_bls12_377_mul_projective_te | 456.99 us | 90.48 us | 5.05x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_10 | 8.16 ms | 2.40 ms | 3.40x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_100 | 69.40 ms | 19.32 ms | 3.59x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_32 | 23.80 ms | 6.57 ms | 3.62x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_55 | 40.36 ms | 11.16 ms | 3.62x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_77 | 55.27 ms | 15.26 ms | 3.62x |
| ed_on_bls12_381_bandersnatch_msm_te_x_10 | 9.35 ms | 2.40 ms | 3.89x |
| ed_on_bls12_381_bandersnatch_msm_te_x_100 | 72.28 ms | 18.90 ms | 3.82x |
| ed_on_bls12_381_bandersnatch_msm_te_x_32 | 25.65 ms | 6.55 ms | 3.92x |
| ed_on_bls12_381_bandersnatch_msm_te_x_55 | 42.85 ms | 10.77 ms | 3.98x |
| ed_on_bls12_381_bandersnatch_msm_te_x_77 | 57.39 ms | 14.83 ms | 3.87x |
| ed_on_bls12_381_bandersnatch_mul_affine_sw | 1.10 ms | 303.11 us | 3.63x |
| ed_on_bls12_381_bandersnatch_mul_affine_te | 1.05 ms | 254.28 us | 4.14x |
| ed_on_bls12_381_bandersnatch_mul_projective_sw | 616.88 us | 115.81 us | 5.33x |
| ed_on_bls12_381_bandersnatch_mul_projective_te | 496.44 us | 98.63 us | 5.03x |

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
| ietf_vrf_verify | 2.10 ms | 598.77 us | 3.51x |
| ring_vrf_accumulate_and_commit_x_1 | 3.15 ms | 1.19 ms | 2.64x |
| ring_vrf_accumulate_and_commit_x_13 | 12.28 ms | 2.58 ms | 4.76x |
| ring_vrf_accumulate_and_commit_x_25 | 19.12 ms | 4.04 ms | 4.73x |
| ring_vrf_accumulate_and_commit_x_37 | 29.54 ms | 5.42 ms | 5.45x |
| ring_vrf_accumulate_and_commit_x_50 | 34.77 ms | 6.86 ms | 5.07x |
| ring_vrf_accumulate_x_1 | 3.11 ms | 1.35 ms | 2.30x |
| ring_vrf_accumulate_x_13 | 11.98 ms | 2.77 ms | 4.32x |
| ring_vrf_accumulate_x_25 | 19.07 ms | 4.22 ms | 4.52x |
| ring_vrf_accumulate_x_37 | 29.54 ms | 5.10 ms | 5.79x |
| ring_vrf_accumulate_x_50 | 34.75 ms | 6.43 ms | 5.40x |
| ring_vrf_commit | 27.12 us | 40.35 us | 0.67x |
| ring_vrf_verify | 28.37 ms | 14.58 ms | 1.95x |
| ring_vrf_verify_batch_x_1 | 28.24 ms | 15.23 ms | 1.85x |
| ring_vrf_verify_batch_x_4 | 46.59 ms | 20.52 ms | 2.27x |
| ring_vrf_verify_batch_x_8 | 69.56 ms | 34.03 ms | 2.04x |
| ring_vrf_verify_batch_x_12 | 89.44 ms | 42.04 ms | 2.13x |
| ring_vrf_verify_batch_x_16 | 106.18 ms | 52.80 ms | 2.01x |
