module Routing exposing (..)


import Navigation exposing (Location)
import Models exposing (Route(..), PlayerId)
import UrlParser exposing (..)


routes : Parser (Route -> a) a
routes =
  oneOf
    [ map PlayersRoute top
    , map PlayerRoute (s "players" </> string)
    , map PlayersRoute (s "players")
    ]


playersPath : String
playersPath =
  "#/players"


playerPath : PlayerId -> String
playerPath id =
  "#/players/" ++ id


parseLocation : Location -> Route
parseLocation location =
  case (parseHash routes location) of
    Just route ->
      route

    Nothing ->
      NotFoundRoute
