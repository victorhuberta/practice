import Html exposing (Html, button, span, text, div)
import Html.Events exposing (onClick)


main =
  Html.beginnerProgram { model = model, update = update, view = view }


type alias Model = Int

model : Model
model =
  0


type Msg = Increment | Decrement | Reset

update : Msg -> Model -> Model
update msg model =
  case msg of
    Increment ->
      model + 1

    Decrement ->
      model - 1

    Reset ->
      0


view : Model -> Html Msg
view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , span [] [ text (toString model) ]
    , button [ onClick Increment ] [ text "+" ]
    , div []
        [ button [ onClick Reset ] [ text "Reset" ] ]
    ]
