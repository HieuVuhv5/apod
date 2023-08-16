# Project: Astronomy Picture of the Day - NASA (APOD)

## Description
This project is developed as part of CS410P - Rust Web Development by Hieu Vu. It is a web application that provides users with access to the Astronomy Picture of the Day (APOD) using the NASA API. The project is built using Rust's Axum framework and Sqlx for database interaction, with Postgres configured using Docker.

## Table of Contents
1. [GitHub Repository and Demo](#github-repository-and-demo)
2. [How the Project Works](#how-the-project-works)
3. [Website Features](#website-features)
4. [Other Features](#other-features)

## GitHub Repository and Demo
- GitHub Repository: [https://github.com/HieuVuhv5/apod](https://github.com/HieuVuhv5/apod)
- Online Demo: [www.tonghopmoi.com](http://www.tonghopmoi.com)

## How the Project Works
The project's main functionality revolves around fetching and displaying Astronomy Pictures of the Day. The workflow is as follows:
1. When a user searches for an APOD, the system first checks its database for the result.
2. If no result is found in the database, the system queries the NASA API to retrieve the APOD for the requested date.
3. The retrieved result is then added to the system's database for future use and displayed to the user.
4. Over time, as more users interact with the system, the database becomes more comprehensive, improving the user experience.

## Website Features
The website offers the following features:

1. User Registration:
   - Users are required to register for an account using their email address.
   - ![User Registration](/report_apod/register.png)

2. User Galleries:
   - Registered users can create and customize their own galleries to store APODs.
   - ![User Gallery](/report_apod/gallery.png)

3. Slide Show:
   - Users with their own galleries can enjoy a slide show of the Astronomy Pictures in their gallery.
   - ![Slide Show](/report_apod/slide.png)

4. Search:
   - Users can search for APODs from any date using the search page.
   - ![Search](/report_apod/search.png)

## Other Features
The project also includes additional features such as:
- Duplicate email checking during registration.
- Login and logout functionality, tested thoroughly.

## Getting Started
To set up and run the project locally, follow these steps:

1. Clone the GitHub repository: `git clone https://github.com/HieuVuhv5/apod.git`
2. Navigate to the project directory: `cd apod`
3. Install dependencies: `cargo install`
4. Configure your Postgres database using Docker.
5. Run the project: `cargo run`

Make sure to refer to the project's GitHub repository for detailed setup instructions and any additional information.

## License
This project is licensed under the [MIT License](LICENSE).

---
*Note: The images used in this README are for illustration purposes only and may not accurately represent the actual design of the website.*
