import Html exposing (Html, div, h2, img, button, text, select, option)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onClick)
import Http
import Json.Decode as Decode


main =
  Html.program
    { init = init "cat"
    , update = update
    , view = view
    , subscriptions = subscriptions
    }


type alias Model =
  { topic : String
  , gifUrl : String
  , errMessage : String
  }


type Msg
  = SelectTopic String
  | MorePls
  | NewGif (Result Http.Error String)


init : String -> (Model, Cmd Msg)
init topic =
  (Model topic "" "", getRandomGif topic)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    SelectTopic newTopic ->
      ({ model | topic = newTopic }, Cmd.none)

    MorePls ->
      (model, getRandomGif model.topic)

    NewGif (Ok url) ->
      ({ model | gifUrl = url }, Cmd.none)

    NewGif (Err Http.Timeout) ->
      ({ model | errMessage = "Oh no! We got a timeout" }, Cmd.none)

    NewGif (Err Http.NetworkError) ->
      ({ model | errMessage = "Random network error" }, Cmd.none)

    NewGif (Err _) ->
      ({ model | errMessage = "Unknown Http error" }, Cmd.none)


view : Model -> Html Msg
view model =
  div []
    [ h2 [] [ text model.topic ]
    , img [ src model.gifUrl, style [("display", "block")] ] []
    , select [ onInput SelectTopic ]
      [ option [ value "cat" ] [ text "Cate" ]
      , option [ value "dog" ] [ text "Doggo" ]
      , option [ value "bird" ] [ text "Birb" ]
      ]
    , button [ onClick MorePls ] [ text "More Please" ]
    , div [ style [("color", "#f00")] ] [ text model.errMessage ]
    ]


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none


getRandomGif : String -> Cmd Msg
getRandomGif topic =
  let
    url =
      "https://api.giphy.com/v1/gifs/random?api_key=dc6zaTOxFJmzC&tag=" ++ topic

    request =
      Http.get url decodeGifUrl
  in
    Http.send NewGif request


decodeGifUrl : Decode.Decoder String
decodeGifUrl =
  Decode.at ["data", "image_url"] Decode.string
