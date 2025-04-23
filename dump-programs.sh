#!/bin/bash

solana program dump -u https://api.mainnet-beta.solana.com CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d target/deploy/mpl_core.so 

COPYFILE_DISABLE=1 solana-test-validator --bpf-program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d target/deploy/mpl_core.so --reset