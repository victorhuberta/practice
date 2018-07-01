import Html exposing (Html, div, button, text)
import Html.Events exposing (onClick)
import Random


main =
  Html.program
    { init = init
    , update = update
    , view = view
    , subscriptions = subscriptions
    }


type alias Model =
  { dieFace0 : Int
  , dieFace1 : Int
  }


type Msg
  = Roll
  | NewFace0 Int
  | NewFace1 Int


init : (Model, Cmd Msg)
init =
  (Model 1 1, Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Roll ->
      (model, Cmd.batch [rand1to6 NewFace0, rand1to6 NewFace1])

    NewFace0 face ->
      ({ model | dieFace0 = face }, Cmd.none)

    NewFace1 face ->
      ({ model | dieFace1 = face }, Cmd.none)


view : Model -> Html Msg
view model =
  div []
    [ div [] [ text (toString model.dieFace0) ]
    , div [] [ text (toString model.dieFace1) ]
    , button [ onClick Roll ] [ text "Roll" ]
    ]


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none


rand1to6 tagger=
  Random.generate tagger (Random.int 1 6)
