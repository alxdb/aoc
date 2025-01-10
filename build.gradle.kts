plugins {
    id("org.jetbrains.kotlin.jvm")
}

group = "alxdb.me"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
}

tasks {
    test {
        useJUnitPlatform()
    }
}
kotlin {
    jvmToolchain(21)
}