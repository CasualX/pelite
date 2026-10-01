file(SHA256 "${SOURCE}" source_hash)
file(SHA256 "${BINARY}" binary_hash)
file(WRITE "${OUTPUT}" "{\n  \"architecture\": \"${ARCH}\",\n  \"compiler_id\": \"${COMPILER_ID}\",\n  \"compiler_version\": \"${COMPILER_VERSION}\",\n  \"frontend\": \"${FRONTEND}\",\n  \"target_system\": \"${TARGET_SYSTEM}\",\n  \"configuration\": \"${CONFIG}\",\n  \"source_sha256\": \"${source_hash}\",\n  \"binary_sha256\": \"${binary_hash}\"\n}\n")
