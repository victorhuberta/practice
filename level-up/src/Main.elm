module Main exposing (..)


import Html exposing (Html)
import Models exposing (Model, initialModel)
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
  (initialModel, Cmd.none)


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none
