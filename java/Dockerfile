FROM openjdk:17-jdk-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY gradlew .
COPY gradle gradle
COPY build.gradle .

RUN chmod +x ./gradlew
RUN ./gradlew --version --no-daemon

COPY src src

RUN ./gradlew bootJar --no-daemon

EXPOSE 8003

RUN mv build/libs/*.jar app.jar

CMD ["java", "-jar", "app.jar"]