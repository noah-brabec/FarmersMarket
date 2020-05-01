module Login exposing (..)

import Bootstrap.CDN as CDN
import Bootstrap.Grid as Grid
import Bootstrap.Grid.Col as Col
import Browser exposing (Document)
import Html exposing (Html, div, h4, text)


main : Program () Model Msg
main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    { title : String
    , number : Int
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { title = "Farmers Market"
      , number = 1
      }
    , Cmd.none
    )


type Msg
    = Option1


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        option1 ->
            ( model, Cmd.none )


type alias Document msg =
    { title : String
    , body : List (Html msg)
    }


viewBody : Model -> Html msg
viewBody model =
    Grid.container []
        [ CDN.stylesheet
        , Grid.row []
            [ Grid.col [ Col.xs ] [ text "One" ]
            , Grid.col [ Col.xs ] [ text "Two" ]
            , Grid.col [ Col.xs ] [ text "Three" ]
            ]
        ]


view : Model -> Document msg
view model =
    { title = model.title
    , body = [ div [] [ viewBody model ] ]
    }
