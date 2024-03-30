# monmon

monmon runs in the background to notify you of cheap cryptocurrency rates.
It is recommended to add monmon to your start-up programs.

## How it works

monmon queries CoinAPI.io for the current rate of your chosen cryptocurrency and notifies you when it falls below your custom threshold.
[Get your API key in seconds here!](https://www.coinapi.io/get-free-api-key)

## Configuration

Here is an example of the configuration file "config.json". It needs to reside next to the executable.

> Set the notification duration to 0 to disable the timeout.

    {
        "api_key": "YOUR API KEY HERE",
        "currency": "XMR",
        "fiat_unit": "EUR",
        "notify_at": 110.0,
        "scan_delay_in_min": 20,
        "notif_dur_in_secs": 5
    }

monmon was tested with XMR and BTC with the fiat units set to EUR and USD.
