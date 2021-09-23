<br />
<p align="center">
  <a href="https://github.com/StarPlatin4m/auto-trader">
    <img src="img/logo.jpg" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Cryptocurrency Trading Robot</h3>

  <p align="center">
    A fully featured and customizable cryptocurrency trading bot for the binance exchange
    <br />
    <br />
    <br />
    <a href="https://discord.com/invite/eKqScrCUCr">Discord</a>
    ·
    <a href="https://github.com/StarPlatin4m/auto-trader/issues">Report Bug</a>
    ·
    <a href="https://github.com/StarPlatin4m/auto-trader/issues">Request Feature</a>

  </p>
</p>

<!-- TABLE OF CONTENTS -->
<details open="open">
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
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

There are many great automated cryptocurrency bots for binance. I decided to make my own since almost all of them are entirely based of fixed percentages for all of the cryptocurrencies. My goal was to create a bot that can adapt to the curves of individual cryptocurrencies, and thus this project was born

Well, How does it determine the best points to buy and sell?
It runs simulations with various values based on past data, from this it determines the values that maximize profit. It basically learns from the past information in an attempt to predict the future.

### Features

Features are optional, and can be turned on or off in `config.toml`

0. Sane default settings, So you will be making money out of the box
1. Will buy coins based on specific thresholds
2. Will purchase new coin listings on the binance exchange
3. Volatility trading mode to profit off fluctuations in the cryptomarket

### Built With

This program was written in the rust programming language and uses the binance api, as well as discord integration for notifications

- [Rust](https://www.rust-lang.org)
- [binance-rs](https://github.com/wisespace-io/binance-rs)
- [serenity](https://github.com/serenity-rs/serenity)

<!-- GETTING STARTED -->

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

To get the program up and running, you will need the following

- [Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Get an API Key at [https://www.binance.com/](https://www.binance.com/)
   - click on your account icon
   - Select the api management section
   - Follow the prompts to create an api key
   - Provide the necessary permissions
   - You will recieve a token and a secret, keep these safe
2. Clone the repo

   ```sh
   git clone https://github.com/StarPlatin4m/auto-trader
   ```

   Alternatively, Download as a zip archive and extract

3. Enter your API Keys in `config.toml`

   ```toml
   token = "token here"
   secret = "secret here"
   ```

4. Optionally. Get a discord api token from the [discord developer website](https://discord.com/developers/applications) and create an application. Store the token key in config.toml in order to recieve discord based notifications whenever the bot does something.

   ```toml
   # discord api token
   # leave empty for no discord notifications
   discord_token = "token here"
   ```

   also store the channel id of the channel to send updates to. This is an array, so we can send to multiple channels

   ```toml
    # discord channels to send notifications to
    channel_ids = [886912998831181835,774344735501844491]
   ```

   to get the channel id, right click on the channel name and click copy id. You may have to enable developer mode for this to work.

<!-- USAGE EXAMPLES -->

## Usage

To run the program

1. Enter the program directory within a terminal
2. Run the command
   ```sh
    cargo run --release
   ```
3. Ensure that you are running it from the same directory as config.toml
4. Optionally, copy the executable file from the `/target/release/` directory and run it directly. `config.toml` must appear in the same directory

<!-- ROADMAP -->

## Customisation

By default only watches ADA and ETH; To customise it to coins of your choice

1. Edit config.toml
2. Add the tokens of your choice in the specified format

   ```toml
   [[tokens]]
   symbol = "ADAUSDT"
   token = "ADA"
   base = "USDT"
   perc_up = 1.798
   time_up = 140
   perc_down = -1.781
   time_down = 146
   ratio = 20.0
   ```

   - time_down ~ the time range over which to determine to sell( in 5 min intervals)
   - perc_down ~ the percentage at which to sell
   - For example: It will sell after ADA goes down -1.781% over the past 146 x 5 minute intervals (146\*5/60 = x hours)
   - time_up ~ the time range over which to determine to buy( in 5 min intervals)
   - perc_up ~ the percentage at which to buy
   - For example: It will buy after ADA goes up 1.798% over the past 146 x 5 minute intervals (140\*5/60 = x hours)

   - ratio is the percentage of your total balance to invest in that currency
   - 100 => 100% of total worth etc.
   - symbol is the symbol of the pair
   - fiat is currency to sell to
   - token is the currency to buy

3. How do i determine these values? Easy, i wrote another program to do it. It will soon be integrated with this one

## Roadmap

See the [open issues](https://github.com/StarPlatin4m/auto-trader/issues) for a list of proposed features (and known issues).

- Add volatility trading mode(WIP)

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses. Cryptocurrency investment is subject to high market risk.

<!-- LICENSE -->

## License

Distributed under the GPL-3.0 License. See `LICENSE` for more information.

<!-- CONTACT -->

## Contact

Kival Mahadew - kivalm(at)protonmail.com

Project Link: [https://github.com/StarPlatin4m/auto-trader](https://github.com/StarPlatin4m/auto-trader)
