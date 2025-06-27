FROM openjdk:17-jdk-slim

WORKDIR /app

# Install curl, unzip, and git
RUN apt-get update && apt-get install -y curl unzip git && rm -rf /var/lib/apt/lists/*

# Install sbt
RUN curl -L -o sbt.zip https://github.com/sbt/sbt/releases/download/v1.7.3/sbt-1.7.3.zip && \
    unzip sbt.zip && \
    mv sbt /opt/sbt && \
    ln -s /opt/sbt/bin/sbt /usr/local/bin/sbt && \
    rm sbt.zip

# Copy build files
COPY build.sbt .
COPY project/ project/

# Download dependencies first (this layer will be cached)
RUN sbt -J-Xmx1024M -J-Xms512M update

# Copy source code
COPY src/ src/

# Build the application
RUN sbt -J-Xmx1024M -J-Xms512M stage

EXPOSE 8001

CMD ["target/universal/stage/bin/todo-scala"]