Since in OSX 10.11 openssl is no longed included
You will need to install it via brew and add these env vars

*bash*
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include

*fish*
set -x OPENSSL_LIB_DIR (brew --prefix openssl)/lib
set -x OPENSSL_INCLUDE_DIR (brew --prefix openssl)/include
