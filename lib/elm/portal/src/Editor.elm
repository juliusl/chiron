module Editor exposing (viewMonacoEditor, viewMultilineEditor)

import Element exposing (Element)
import Element.Input
import Element.Font as Font
import Html exposing (node)
import Html.Attributes exposing (attribute)


type alias Model =
    { text : String, language : String }

viewMonacoEditor : Model -> Element msg
viewMonacoEditor model =
    Element.html
        (node "code-editor"
            [ attribute "value" model.text
            , attribute "language" model.language
            ]
            []
        )


viewMultilineEditor : (String -> msg) -> Model -> Element msg
viewMultilineEditor onChange model =
    Element.Input.multiline 
        [ Font.size 14
        , Font.family [ Font.monospace] ]
        { onChange = onChange
        , text = model.text
        , placeholder = Nothing
        , label = Element.Input.labelHidden "Editor"
        , spellcheck = False
        }
