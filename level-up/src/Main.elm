module Main exposing (..)


import Html exposing (Html)
import Models exposing (Model)
import Msgs exposing (Msg)
import Update exposing (update)
import View exposing (view)


main : Program Never Model Msg
main =
  Html.program
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }


init : (Model, Cmd Msg)
init =
  ("Hello, world!", Cmd.none)


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none
