module Editor exposing (viewCodeEditor)

import Element exposing (..)
import Element.Input
import Element.Font as Font
import Html exposing (node)
import Html.Attributes exposing (attribute)

type alias Model =
    { text : String, language : String }

viewCodeEditor : { onDispatchSave: msg, onSave: (String -> msg) } -> { enableMonaco : Bool, visible: Bool } -> { language : String, text : String } -> Element msg
viewCodeEditor msgs settings editor =
    let
        visible =
            settings.visible

        enableMonaco =
            settings.enableMonaco
        onDispatch =
            msgs.onDispatchSave
        onSave =
            msgs.onSave
    in
    if visible then 
            if enableMonaco then
                Element.column [ width fill, height fill ]
                [   Element.Input.button []
                    { onPress = Just onDispatch
                    , label = Element.text "Render"
                    }
                ,viewMonacoEditor editor
                ]
        else
        viewMultilineEditor onSave editor
    else
        Element.text ""

viewMonacoEditor : Model -> Element msg
viewMonacoEditor model =
    Element.html
        (node "code-editor"
            [ attribute "value" model.text
            , attribute "language" model.language
            ] [ ]
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
