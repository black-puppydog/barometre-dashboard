# Dashboad "Mon Baromètre"

A simple but hopefully useful dashboard for the "Baromètre Vélo" of the fub.

**Do the survey first!**
If you haven't done so yet, head on over here to fill it out.
Then you can look at dashboards. 🙂

https://www.barometre-velo.fr

This repository hosts a dashboard that's complementary to the official [map][map] that the fub provides.
You can use this dashboard hosted here:

https://barometre.toulon-var-deplacements.fr/?prefix=83

(change the prefix to your liking, or remove it for a search mask)
Here's a screenshot for the prefix `83` (the Var département):

![screenshot.jpg](doc/screenshot.jpg)

[map]:https://www.barometre-velo.fr/dashboard.php



## Development

This code uses [Rust][rust] and [dioxus][dioxus] and is based off of the dioxus "Jumpstart" template.
It includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

[rust]: https://www.rust-lang.org/
[dioxus]: https://dioxuslabs.com/

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss@3 -i ./input.css -o ./assets/tailwind.css --watch
```

**Note** the `@3` in that command; dioxus is not currently compatible with v4 of tailwind.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

### Offline development

There is *rudimentary* offline development support.
By default, the browser will fetch the fub REST API upon load to retrieve up-to-date data.
However, when working from a train or such, and generally when reloading often and rapidly, this may be undesirable.
To avoid this, serve the `assets` folder using a web server of your choice (I use [caddy]) and make sure that it's CORS header is set to allow requests from all sources.
Then, launch your development server with

```terminal
dx serve --features local-dev
```

This will pull [`assets/example_response.json`](assets/example_response.json) on every load,
speeding up your development and saving bandwidth for you and the fub.

If you want to update the data, just `curl` the current state (the `| jq` is optional of course) :

```terminal
curl 'https://www.barometre-velo.fr/stats/progress.geojson'  | jq > example_response.json
```
