// Custom signing configuration
// This file is applied after build.gradle.kts to add signing support
// It won't be overwritten by tauri android init

android {
    signingConfigs {
        val keystoreFile = file("release-key.keystore")
        if (keystoreFile.exists()) {
            create("release") {
                storeFile = keystoreFile
                storePassword = System.getenv("KEYSTORE_PASSWORD") ?: ""
                keyAlias = System.getenv("KEY_ALIAS") ?: ""
                keyPassword = System.getenv("KEY_PASSWORD") ?: ""
            }
        }
    }
    
    buildTypes {
        getByName("release") {
            val signingConfig = signingConfigs.findByName("release")
            if (signingConfig != null) {
                signingConfig = signingConfig
            }
        }
    }
}
