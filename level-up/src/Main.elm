module Main exposing (..)


import Navigation exposing (Location)
import Models exposing (Model, initialModel)
import Msgs exposing (Msg)
import Update exposing (update)
import View exposing (view)
import Commands exposing (fetchPlayers)
import Routing


main : Program Never Model Msg
main =
  Navigation.program Msgs.OnLocationChange
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }


init : Location -> (Model, Cmd Msg)
init location =
  let
    currentRoute =
      Routing.parseLocation location
  in
    (initialModel currentRoute, fetchPlayers)


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none
