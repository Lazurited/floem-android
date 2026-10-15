plugins {
    id("com.android.application")
}

android {
    namespace = "dev.lazurited.counter_simple"
    compileSdk = 34

    defaultConfig {
        applicationId = "dev.lazurited.counter_simple"
        minSdk = 26
        targetSdk = 28
        versionCode = 1
        versionName = "0.1.0"

        ndk {
            abiFilters += "arm64-v8a"
        }
    }

    sourceSets["main"].jniLibs.srcDirs("../../jniLibs")

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
