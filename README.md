# Sanbapolis Arena Management System

Welcome to the repository for everything regarding the management system for the Sanbapolis Arena in Trento.

### Table of Contents

`// TODO`

## Introduction

The Sanbapolis Arena has 13 cameras deployed to record sport events from multiple perspectives.
The goal of this project is to allow the teams to easily use the cameras to record matches or training sessions and then use analytics tools to extract valuable data to improve performance.

### System Architecture

The system is composed by a Rust backend which exposes web APIs for client interaction with a Rocket server. Data is stored in a MariaDB database (a fork of MySQL).

This architecture gives the option to implement various frontend applications, such as a webapp, a mobile app, a Telegram bot, etc.

## Project Structure

This section exlains how the project is organised and gives some advice on how to expand this project while keeping it well structured.

### Backend

The Rust backend is divided in several packages following the Clean Architecture guideline. Instead of building a monolithic software (which would become a nightmare to expand and maintain), each package focuses on one aspect of the system (also called layer).

The packages are:

-   **`api`**

    This package contains all the code that enables the interaction with the `application` package through web APIs.

    The APIs are documented with the OpenAPI 3.1 specification and a Rust wrapper for Swagger-UI. Run the project and visit http://localhost:8000/swagger-ui to take a look at the documentation.

-   **`application`**

    This package contains the business logic that performs operations on data stored and provided by the `domain` package.

-   **`domain`**

    This package contains all the models and schemas used by Diesel to send and retrieve data from the MariaDB database.

-   **`infrastructure`**

    This package is used to keep all files used by dependencies, such as Diesel's migrations.

-   **`shared`**

    Any data or code used in multiple parts of the project is kept here.

The dependencies between packages are organized so that the code follows the principles of Clean Architecture.

### Frontend

`//TODO when the frontend development is ongoing`

## Install and run

In addition to this repo, to run the project you will need to setup a database and (optionally) a RSTP camera (which can be emulated with VLC).

### Database Setup

1.  Download and install a recent version of [MariaDB](https://mariadb.org/download/) for your operating system.

2.  Open a terminal and use these commands to create the database:

    a. Connect to the MariaDB server with

        sudo mysql -u root -p

    and enter the password for the root user.

    b. Create the database with

        CREATE DATABASE sanbapolis;

    c. Create a user that Diesel will use to operate on the database:

        CREATE USER 'sanbapolis_user'@'localhost' IDENTIFIED BY '<password>';

        GRANT ALL PRIVILEGES ON sanbapolis.* TO 'sanbapolis_user'@'localhost';

        FLUSH PRIVILEGES;

    You will need to add the username and password to the `.env` file of the project.

    d. Exit the DB console

        EXIT;

    e. (Optional) You might want to install a client to inspect the database for development purposes such as [DBeaver](https://dbeaver.io/)

3.  In the root folder of the project create a .env file and add the following variables:

        DATABASE_URL=mysql://sanbapolis_user:<password>@localhost/sanbapolis
        JWT_SECRET=<jwt_secret>     # The key used to encrypt JWT tokens
        JWT_DURATION_SECONDS=900    # 15 minutes of validity for every token generated

    Make sure to set the same password you chose for the user in step 2c.

4.  Follow the instructions to install [Diesel CLI](https://diesel.rs/guides/getting-started.html#installing-diesel-cli)

    > At the top of the page make sure to select the guide for MySQL (MariaDB is cross-compatible with MySQL syntax and drivers).

5.  Navigate to the `infrastructure` crate and use the following command to setup the database with Diesel

        diesel migration run

    To make sure everything works as expected, you can also revert and redo the migrations with this command

        diesel migration redo

6.  Now that the database structure is present, you can run the project and, for instance, signup as a new user and create a sports club and a team. To learn more about what APIs you can use, visit the `/swagger-ui` page while the project is running.

### Camera Setup

`//TODO complete installation documentation`
