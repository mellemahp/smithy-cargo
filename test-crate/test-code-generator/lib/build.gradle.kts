
description = "Simple rust code generator for testing purposes"

plugins {
    `maven-publish`
    `java-library`
}

dependencies {
    implementation("software.amazon.smithy:smithy-codegen-core:1.55.0")
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "dev.hmellema.smithy.rust"
            artifactId = "test-code-generator"
            version = "1.0.0"

            from(components["java"])
        }
    }
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()
}
