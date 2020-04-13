module Login exposing (..)

import Bootstrap.CDN as Cdn
import Bootstrap.Grid.Col as Col
import Bootstrap.Grid.Row as Row
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
    { statusText : String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { statusText = "Ready"
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
    div []
        [ h4 [] [ text "Hello Port 8000" ]
        ]


view : Model -> Document msg
view model =
    { title = model.statusText
    , body = [ viewBody model ]
    }
