module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (placeholder)
import Html.Events exposing (onClick, onInput)
import Table



-- MAIN


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }



-- MODEL


type alias Model =
    { message : String
    , number : Int
    , entries : List Listing
    , tableState : Table.State
    , query : String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Model "Hello" 1 listings (Table.initialSort "Name") ""
    , Cmd.none
    )



-- UPDATE


type Msg
    = Increment
    | Excite
    | SetTableState Table.State
    | SetQuery String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Increment ->
            ( { model | number = model.number + 1 }
            , Cmd.none
            )

        Excite ->
            ( { model | message = model.message ++ "!" }
            , Cmd.none
            )

        SetTableState newState ->
            ( { model | tableState = newState }
            , Cmd.none
            )

        SetQuery newQuery ->
            ( { model | query = newQuery }
            , Cmd.none
            )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Html Msg
view { number, message, entries, tableState, query } =
    let
        lowerQuery =
            String.toLower query

        acceptableListing =
            List.filter (String.contains lowerQuery << String.toLower << .name) entries
    in
    div []
        [ h1 [] [ text message ]
        , h1 [] [ text (String.fromInt number) ]
        , button [ onClick Increment ] [ text "Increment" ]
        , button [ onClick Excite ] [ text "Get Excited" ]
        , h1 [] [ text "Current Offerings" ]
        , h2 [] [ text "Add an Entry" ]
        , input [ placeholder "Search by Name", onInput SetQuery ] []
        , Table.view config tableState acceptableListing
        ]


config : Table.Config Listing Msg
config =
    Table.config
        { toId = .name
        , toMsg = SetTableState
        , columns =
            [ Table.stringColumn "Name" .name
            , Table.stringColumn "Farm" .farm
            , Table.floatColumn "Price" .price
            , Table.intColumn "Amount" .amount
            , Table.intColumn "Quality" .quality
            ]
        }



-- PRODUCE


type alias Listing =
    { name : String
    , farm : String
    , price : Float
    , amount : Int
    , quality : Int
    }


listings : List Listing
listings =
    [ Listing "Tomato" "Moose Farms" 10.23 2 5
    , Listing "Orange" "Citron Corp" 4.55 10 9
    , Listing "Egg" "Conglomerate of Egg" 0.21 10000 2
    ]
