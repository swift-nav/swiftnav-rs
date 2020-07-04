# Image is built and published from https://github.com/swift-nav/docker-recipes
FROM 571934480752.dkr.ecr.us-west-2.amazonaws.com/swift-tools:2020-04-15-1

RUN sudo apt-get update && sudo apt-get install -y cmake