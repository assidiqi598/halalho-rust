#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="rabbitmq"
DOMAIN="AMQPRS_TEST"
DAYS=36500
KEY_SIZE=4096

mkdir -p "${OUT_DIR}"
cd "${OUT_DIR}"

echo "==> Generating CA"
openssl genrsa -out ca_key.pem ${KEY_SIZE}

openssl req -x509 -new -nodes \
  -key ca_key.pem \
  -sha256 \
  -days ${DAYS} \
  -out ca_certificate.pem \
  -subj "/CN=${DOMAIN}_CA"

echo "==> Creating server config"
cat > server.cnf <<EOF
[req]
distinguished_name = dn
req_extensions = req_ext
prompt = no

[dn]
CN = ${DOMAIN}

[req_ext]
subjectAltName = @alt_names
extendedKeyUsage = serverAuth

[alt_names]
DNS.1 = ${DOMAIN}
DNS.2 = localhost
EOF

echo "==> Generating server key"
openssl genrsa -out server_key.pem ${KEY_SIZE}

echo "==> Generating server CSR"
openssl req -new \
  -key server_key.pem \
  -out server.csr \
  -config server.cnf

echo "==> Signing server certificate"
openssl x509 -req \
  -in server.csr \
  -CA ca_certificate.pem \
  -CAkey ca_key.pem \
  -CAcreateserial \
  -out server_certificate.pem \
  -days ${DAYS} \
  -sha256 \
  -extfile server.cnf \
  -extensions req_ext

echo "==> Creating client config"
cat > client.cnf <<EOF
[req]
distinguished_name = dn
req_extensions = req_ext
prompt = no

[dn]
CN = ${DOMAIN}

[req_ext]
subjectAltName = @alt_names
extendedKeyUsage = clientAuth

[alt_names]
DNS.1 = ${DOMAIN}
EOF

echo "==> Generating client key"
openssl genrsa -out client_key.pem ${KEY_SIZE}

echo "==> Generating client CSR"
openssl req -new \
  -key client_key.pem \
  -out client.csr \
  -config client.cnf

echo "==> Signing client certificate"
openssl x509 -req \
  -in client.csr \
  -CA ca_certificate.pem \
  -CAkey ca_key.pem \
  -CAcreateserial \
  -out client_certificate.pem \
  -days ${DAYS} \
  -sha256 \
  -extfile client.cnf \
  -extensions req_ext

echo "==> Cleaning up"
rm -f *.csr *.cnf *.srl

echo "✅ Certificates generated in ${OUT_DIR}"
