Since in OSX 10.11 openssl is no longed included
You will need to install it via brew and add these env vars

*bash*
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include
export OPENSSL_ROOT_DIR=/usr/local/opt/openssl

*fish*
set -x OPENSSL_INCLUDE_DIR /usr/local/opt/openssl/include
set -x DEP_OPENSSL_INCLUDE /usr/local/opt/openssl/include
set -x OPENSSL_ROOT_DIR /usr/local/opt/openssl
