# Rust Summarization

![CI/CD Pipeline](https://github.com/brycewhit13/rust-miniprojects/actions/workflows/rust.yml/badge.svg)
![Tests](https://github.com/brycewhit13/rust-miniprojects/actions/workflows/tests.yml/badge.svg)

## Description

The goal of this project is to create a microservice where someone can input a block of text, and the output will be a summarized version of the text. This is accomplished using the [distilbart-cnn-6-6 model](https://huggingface.co/sshleifer/distilbart-cnn-6-6) from [Hugging Face](https://huggingface.co/). The result will be an abstractive summary, which means the resulting sentences may not exist in the original text. The information will all come from the text, but it may be rearranged to make the summary more understandable and concise.

## Architecture

## Running the Program

### Locally

1. Run `cargo run --release`
2. Navigate to [http://127.0.0.1:8080/](http://127.0.0.1:8080/)

### Docker

The docker container can be found publically available [here](https://hub.docker.com/r/brycewhit13/rust-summarization).

1. To pull the docker container, run `docker pull brycewhit13/rust-summarization`
2. TO run the docker container, run `docker run -d -p 8080:8080 brycewhit13/rust-summarization:latest`

### Github Release

The binary can be downloaded from the [releases page](https://github.com/brycewhit13/rust-summarization/releases). Selecting the most recent release will usually be the safest option.

### Online
