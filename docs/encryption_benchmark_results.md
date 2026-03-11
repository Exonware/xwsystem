# Encryption Benchmark Results

## Throughput (MB/s)

| Algorithm | 1 KiB | 100 KiB | 1 MiB | 10 MiB |
|-----------|-------|---------|-------|--------|
| aes256-gcm | 170.7 / 240.5 | 3850.8 / 4225.3 | 1496.6 / 1527.8 | 1619.9 / 1412.1 |
| xchacha20-poly1305 | 138.7 / 179.1 | 1672.8 / 1856.5 | 1009.7 / 1019.9 | 989.8 / 992.7 |
| fernet | 38.8 / 68.4 | 153.7 / 347.2 | 129.3 / 220.2 | 141.9 / 222.3 |

(Encrypt / Decrypt MB/s per cell)

## Latency (ms per encrypt/decrypt)

| Algorithm | 64 B | 1 KiB |
|-----------|------|-------|
| aes256-gcm | 0.006 / 0.006 | 0.005 / 0.005 |
| xchacha20-poly1305 | 0.005 / 0.005 | 0.006 / 0.005 |
| fernet | 0.018 / 0.017 | 0.021 / 0.019 |

## Safety ranking (documented)

| Algorithm | Key size | Mode | KDF | Tier |
|-----------|----------|------|-----|------|
| aes256-gcm | 256 | AEAD | PBKDF2/Argon2id | 1 (highest) |
| xchacha20-poly1305 | 256 | AEAD | PBKDF2/Argon2id | 1 (highest) |
| fernet | 128 (effective) | CBC+HMAC | PBKDF2 | 2 (high) |

Tier 1 = highest (AEAD, 256-bit). Tier 2 = high (secure but older construction).
