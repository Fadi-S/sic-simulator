<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![GNU GPLv3 License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">SIC/XE Simulator</h3>

  <p align="center">
    A simulator created to run SIC/XE code.
    <br />
    <br />
    <a href="https://github.com/Fadi-S/sic-simulator/issues">Report Bug</a>
    ·
    <a href="https://github.com/Fadi-S/sic-simulator/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This project is a SIC/XE (Simplified Instructional Computer/Extended Execution) simulator written in the Rust programming language. The simulator aims to replicate the behavior of the SIC/XE architecture, providing a platform to execute SIC/XE assembly language programs.

Note:
This simulator is intended for educational and instructional purposes and may not cover all aspects or edge cases of the SIC/XE architecture.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

[![Rust][rust-shield]][rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

1) Prepare your SIC/XE assembly code and save it in a file with a .asm extension. For example, you can create a file named program.asm and place your assembly code inside it.
2) Load the assembly code into the simulator by running the simulator's executable file followed by the name of the file that contains your SIC/XE code. For example, if the simulator's executable file is named simulator, you would run ./simulator program.asm.

### Prerequisites

You must have cargo and rust installed on your PC

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/Fadi-S/sic-simulator.git
   ```
2. Install Cargo packages
   ```sh
   cargo install
   ```
3. Run program
   ```sh
   cargo run -- code.asm
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create.
Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "feature".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the GNU GPLv3 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Fadi Sarwat - [@fadisarwat_dev](https://twitter.com/fadisarwat_dev) - me@fadisarwat.dev

Project Link: [https://github.com/Fadi-S/sic-simulator](https://github.com/Fadi-S/sic-simulator)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/Fadi-S/sic-simulator.svg?style=for-the-badge
[contributors-url]: https://github.com/Fadi-S/sic-simulator/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Fadi-S/sic-simulator.svg?style=for-the-badge
[forks-url]: https://github.com/Fadi-S/sic-simulator/network/members
[stars-shield]: https://img.shields.io/github/stars/Fadi-S/sic-simulator.svg?style=for-the-badge
[stars-url]: https://github.com/Fadi-S/sic-simulator/stargazers
[issues-shield]: https://img.shields.io/github/issues/Fadi-S/sic-simulator.svg?style=for-the-badge
[issues-url]: https://github.com/Fadi-S/sic-simulator/issues
[license-shield]: https://img.shields.io/github/license/Fadi-S/sic-simulator.svg?style=for-the-badge
[license-url]: https://github.com/Fadi-S/sic-simulator/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/fadi-sarwat-43110a200
[rust-url]: https://www.rust-lang.org
[rust-shield]: https://www.rust-lang.org/static/images/rust-logo-blk.svg