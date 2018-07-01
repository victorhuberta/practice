import Char
import Html exposing (Html, input, div, text, button)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onClick)


main =
  Html.beginnerProgram { model = model, update = update, view = view }


type alias Model =
  { name : String
  , age : String
  , password : String
  , passwordAgain : String
  , validationResult : (String, String)
  }

model : Model
model =
  Model "" "" "" "" ("", "")


type Msg
  = Name String
  | Age String
  | Password String
  | PasswordAgain String
  | Submit


update : Msg -> Model -> Model
update msg model =
  case msg of
    Name name ->
      { model | name = name }

    Age age ->
      { model | age = age }

    Password pass ->
      { model | password = pass }

    PasswordAgain pass ->
      { model | passwordAgain = pass }

    Submit ->
      { model
      | validationResult =
        if not (String.all Char.isDigit model.age) then
          ("red", "Age must be a number")
        else if model.password /= model.passwordAgain then
          ("red", "Passwords do not match")
        else if String.length model.password <= 8 then
          ("red", "Password must be longer than 8 characters")
        else if not
          (  String.any Char.isUpper model.password
          && String.any Char.isLower model.password
          && String.any Char.isDigit model.password)
          then
          ("red", "Password must contain upper case, lower case, and numeric characters")
        else
          ("green", "OK")
      }


view : Model -> Html Msg
view model =
  div []
    [ input [ type_ "text", placeholder "Name", onInput Name ] []
    , input [ type_ "number", placeholder "Age", onInput Age ] []
    , input [ type_ "password", placeholder "Password", onInput Password ] []
    , input [ type_ "password", placeholder "Re-enter Password", onInput PasswordAgain ] []
    , button [ onClick Submit ] [ text "Submit" ]
    , viewValidation model
    ]


viewValidation : Model -> Html Msg
viewValidation model =
  let (color, msg) = model.validationResult in
  div [ style [("color", color)] ] [ text msg ]
