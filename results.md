# pallet_ark_groth16

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_377_groth16_verify | 16.31 ms | 2.86 ms | 5.69x |
| bls12_381_groth16_verify | 15.63 ms | 2.74 ms | 5.71x |
| bw6_761_groth16_verify | 84.06 ms | 10.73 ms | 7.84x |

# pallet_ark_hostcalls

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| bls12_381_msm_g1_x_10 | 10.35 ms | 3.13 ms | 3.31x |
| bls12_381_msm_g1_x_32 | 26.37 ms | 8.69 ms | 3.03x |
| bls12_381_msm_g1_x_55 | 43.23 ms | 14.01 ms | 3.09x |
| bls12_381_msm_g1_x_77 | 57.22 ms | 20.88 ms | 2.74x |
| bls12_381_msm_g1_x_100 | 71.83 ms | 26.02 ms | 2.76x |
| bls12_381_msm_g2_x_10 | 23.74 ms | 7.67 ms | 3.10x |
| bls12_381_msm_g2_x_32 | 63.14 ms | 22.70 ms | 2.78x |
| bls12_381_msm_g2_x_55 | 108.63 ms | 37.45 ms | 2.90x |
| bls12_381_msm_g2_x_77 | 135.82 ms | 48.78 ms | 2.78x |
| bls12_381_msm_g2_x_100 | 182.66 ms | 66.55 ms | 2.74x |
| bls12_381_mul_affine_g1 | 1.57 ms | 1.58 ms | 0.99x |
| bls12_381_mul_affine_g2 | 3.83 ms | 3.86 ms | 0.99x |
| bls12_381_mul_projective_g1 | 784.37 us | 125.89 us | 6.23x |
| bls12_381_mul_projective_g2 | 3.40 ms | 518.23 us | 6.56x |
| bls12_381_pairing | 8.02 ms | 1.98 ms | 4.05x |
| ed_on_bls12_377_msm_te_x_10 | 9.10 ms | 2.59 ms | 3.51x |
| ed_on_bls12_377_msm_te_x_32 | 25.32 ms | 7.37 ms | 3.43x |
| ed_on_bls12_377_msm_te_x_55 | 42.18 ms | 12.42 ms | 3.40x |
| ed_on_bls12_377_msm_te_x_77 | 55.88 ms | 16.22 ms | 3.44x |
| ed_on_bls12_377_msm_te_x_100 | 69.96 ms | 20.70 ms | 3.38x |
| ed_on_bls12_377_mul_affine_te | 1.12 ms | 298.76 us | 3.74x |
| ed_on_bls12_377_mul_projective_te | 489.95 us | 96.99 us | 5.05x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_10 | 8.84 ms | 2.79 ms | 3.17x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_32 | 25.60 ms | 7.49 ms | 3.42x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_55 | 43.00 ms | 12.27 ms | 3.50x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_77 | 58.68 ms | 16.38 ms | 3.58x |
| ed_on_bls12_381_bandersnatch_msm_sw_x_100 | 74.74 ms | 21.34 ms | 3.50x |
| ed_on_bls12_381_bandersnatch_msm_te_x_10 | 9.63 ms | 2.32 ms | 4.15x |
| ed_on_bls12_381_bandersnatch_msm_te_x_32 | 26.65 ms | 6.19 ms | 4.31x |
| ed_on_bls12_381_bandersnatch_msm_te_x_55 | 44.61 ms | 9.96 ms | 4.48x |
| ed_on_bls12_381_bandersnatch_msm_te_x_77 | 54.34 ms | 13.59 ms | 4.00x |
| ed_on_bls12_381_bandersnatch_msm_te_x_100 | 67.69 ms | 19.04 ms | 3.56x |
| ed_on_bls12_381_bandersnatch_mul_affine_sw | 1.22 ms | 329.20 us | 3.69x |
| ed_on_bls12_381_bandersnatch_mul_affine_te | 1.03 ms | 248.23 us | 4.14x |
| ed_on_bls12_381_bandersnatch_mul_projective_sw | 661.87 us | 128.76 us | 5.14x |
| ed_on_bls12_381_bandersnatch_mul_projective_te | 482.62 us | 90.67 us | 5.32x |

# pallet_ark_vrf

| extrinsic | arkworks | substrate | speedup |
|-----------|----------|-----------|---------|
| ietf_vrf_verify | 2.17 ms | 700.12 us | 3.09x |
| ring_vrf_accumulate_and_commit_x_1 | 3.92 ms | 1.67 ms | 2.35x |
| ring_vrf_accumulate_and_commit_x_13 | 19.94 ms | 5.73 ms | 3.48x |
| ring_vrf_accumulate_and_commit_x_25 | 33.92 ms | 10.05 ms | 3.37x |
| ring_vrf_accumulate_and_commit_x_37 | 48.48 ms | 14.07 ms | 3.44x |
| ring_vrf_accumulate_and_commit_x_50 | 59.58 ms | 18.29 ms | 3.26x |
| ring_vrf_accumulate_x_1 | 3.63 ms | 1.63 ms | 2.23x |
| ring_vrf_accumulate_x_13 | 19.64 ms | 6.30 ms | 3.12x |
| ring_vrf_accumulate_x_25 | 31.44 ms | 10.44 ms | 3.01x |
| ring_vrf_accumulate_x_37 | 48.44 ms | 14.69 ms | 3.30x |
| ring_vrf_accumulate_x_50 | 65.76 ms | 18.95 ms | 3.47x |
| ring_vrf_commit | 23.87 us | 40.25 us | 0.59x |
| ring_vrf_verify | 30.63 ms | 18.39 ms | 1.67x |
| ring_vrf_verify_batch_x_1 | 30.88 ms | 19.59 ms | 1.58x |
| ring_vrf_verify_batch_x_4 | 99.47 ms | 53.23 ms | 1.87x |
| ring_vrf_verify_batch_x_8 | 190.14 ms | 100.46 ms | 1.89x |
| ring_vrf_verify_batch_x_12 | 285.28 ms | 140.07 ms | 2.04x |
| ring_vrf_verify_batch_x_16 | 368.93 ms | 184.22 ms | 2.00x |
