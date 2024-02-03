# Gym Regiter API
This project is a simple rest API made in [Rocket rs](https://rocket.rs/) framework for store the register of *Exercises* and *Trainings* of gym. </br>
Besides of allow the sign up of multiples users and the extraction of weight statiscs of these registers.

## Docker
It's used the docker compose to deploy the project and its services. </br>
Depending of your needs, you can deploy the project in develop mode (compose.dev.yml) or production mode (compose.prod.yml). </br>
The command to orchestrate is:
```sh
docker compose -f <compose-file> up
```
### Watch
If you want do this in watch mode, use:
```sh
docker compose -f <compose-file> watch
```
This way, always that the project to be modified, the docker compose rebuilds the image and ups a new container with it.