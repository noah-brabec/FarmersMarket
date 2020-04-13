module BootTest exposing (main)

import Bootstrap.CDN as CDN
import Bootstrap.Grid as Grid
import GoogleMaps.Map as Map
import Html exposing (Html, text)


mapView : String -> Html Msg
mapView apiKey =
    Map.init apiKey
        |> Map.toHtml


type Msg
    = Message


main =
    Grid.container []
        [ CDN.stylesheet -- creates an inline style node with the Bootstrap CSS
        , Grid.row []
            [ Grid.col []
                [ text "Column 1" ]
            , Grid.col []
                [ text "Column 2" ]
            ]
        , Grid.row []
            [ Grid.col []
                [ mapView "AIzaSyAkNCeNZNQ4PHWhp4bLzeoVyZULwbXMizc" ]
            ]
        ]
