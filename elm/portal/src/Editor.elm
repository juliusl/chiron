module Editor exposing (..)

import Html exposing (node)
import Html.Attributes exposing (attribute)
import Element exposing (Element)
import Element.Input

type alias Model 
    = { text: String, language: String }

viewMonacoEditor : Model -> Element msg
viewMonacoEditor model =
    Element.html 
        ( node "code-editor" 
            [ attribute "value" model.text
            , attribute "language" model.language
            ] [])

viewMultilineEditor : (String -> msg) -> Model -> Element msg 
viewMultilineEditor onChange model = 
    Element.Input.multiline [] 
        { onChange = onChange
        , text = model.text
        , placeholder = Nothing
        , label = Element.Input.labelHidden "Editor"
        , spellcheck = False
        }