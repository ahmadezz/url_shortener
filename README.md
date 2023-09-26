# URL SHORTENER

In this document, you will find instructions on how to run the URL shortener app. The app has two endpoints ```/getShortUrl```, one of which redirects the short URL to the long URL and increases the visit count. The app by default works on ```localhost:8080```, ```0.0.0.0:8080```, and ```127.0.0.1:8080``` network. This ```base_url``` is set ```<http://tier.app>``` in the app's Docker container environment variables and ```docker-compose.yml``` ```base_url``` can be changed to any other valid URL base, like ```<http://we.we>```.  The redirect endpoint (which is a bonus functionality for this app) works when the generated ID is extracted from the short URL, which comes after the base URL, and accessed via ```localhost:8080```

## Key generation for short URLs

Nanoid was used to generate the unique IDs with a variable length between 1 and 7 characters to keep them short. Nanoid uses base 64 encoding, which means each character can be any of the following characters:

```bash
['_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
```

total_combinations = 4398046511104 + 1073741824 + 1073741824 + 1048576 + 262144 + 4096 + 64
Total number of IDs available = 4,400,195,309,632

It would take approximately 24445529.50 hours or 2790.59 years to exhaust all combinations at a rate of 180,000 requests per hour or 50 requests per second.

## Database

The app connects to a Postgres database with a connection string of ```postgres://user:OpenSesame@localhost:5432/shortner```. The database contains two tables ```urls``` and ```stats```. ```urls``` table stores generated unique IDs and their mapped long URLs. ```stats``` stores IDs, and the visits count for that ID.

## Installation Guide

To run this project, you'll need to have Docker, Docker Compose, and Rust installed on your system. Follow the instructions below to install these dependencies:

### 1. Docker

Docker is a containerization platform that allows you to run applications in isolated environments called containers.

**Install Docker:**

- Linux: Follow the installation instructions for your specific Linux distribution [here](https://docs.docker.com/engine/install/).
- Windows: Download and install Docker Desktop for Windows [here](https://docs.docker.com/desktop/install/).
- macOS: Download and install Docker Desktop for Mac [here](https://docs.docker.com/desktop/install/).

### 2. Docker Compose

Docker Compose is a tool for defining and running multi-container Docker applications.

**Install Docker-Compose:**

Docker Compose is often included with Docker Desktop installation on Windows and macOS. For Linux and other installations, follow the instructions here.

-- [Install Docker Compose](https://docs.docker.com/compose/install/)

### 3. Rust

**Install Rust:**
This project is written in Rust. You need to have Rust installed along with the Cargo package manager. You can install Rust by following the official Rust installation guide:

-- [Install Rust](https://www.rust-lang.org/learn/get-started)

## HOW TO

Use the terminal to run the following commands:

## How to run the app in docker

```bash

docker-compose up

```

## How to stop the app in docker

```bash

docker-compose down 

```

## How to run the unit tests

```bash

cargo test

```

## How to send a getShortUrl request example

Use a new terminal to send a request to the endpoint ```/getShortUrl```. Example:

```bash

curl -X 'POST' \
  'http://0.0.0.0:8080/getShortUrl' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "long_url": "http://www.stackoverflow.com"
}'

```

Replace the value of ```"long_url"``` with any other valid website to generate the short URL for it. Example ```"long_url": "http://www.cnbc.com"```

## How to send a redirect request example

Extract the ID from the returned response to the ```/getShortUrl``` request. Example:  From```<http://tier.app/iATXH_>``` take ```iATXH_```

Then go to the browser and surf, ```<http://0.0.0.0:8080/iATXH_>``` and the request will be redirected to the long URL stored in the database if found and

visits_count in the stats table will be incremented by one.
