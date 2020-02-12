module Login exposing (Model, Msg, init, subscriptions, update, view)

import Browser
import Html exposing (..)


main : Program flags Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    { key : Nav.Key
    , url : Url.Url
    , property : propertyType
    }


init : flags -> ( Model, Cmd Msg )
init flags =
    ( Model modelInitialValue, Cmd.none )


type Msg
    = Msg1
    | Msg2


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Msg1 ->
            ( model, Cmd.none )

        Msg2 ->
            ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


view : Model -> Html Msg
view model =
    div []
        [ text "New Element" ]
