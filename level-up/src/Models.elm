module Models exposing (..)


import RemoteData exposing (WebData)


type Route
  = PlayersRoute
  | PlayerRoute PlayerId
  | NotFoundRoute


initialModel : Route -> Model
initialModel route =
  { players = RemoteData.Loading
  , route = route
  }


type alias Model =
  { players : WebData (List Player)
  , route : Route
  }


type alias Player =
  { id : PlayerId
  , name : String
  , level : Int
  }


type alias PlayerId =
  String
