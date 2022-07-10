module Editor exposing (viewCodeEditor)

import Element exposing (Element)
import Element.Input
import Html exposing (node)
import Html.Attributes exposing (attribute)


type alias Model =
    { text : String, language : String }

viewCodeEditor : { enableMonaco : Bool, model : Model } -> Element Msg
viewCodeEditor settings =
    let
        enableMonaco =
            settings.enableMonaco

        model =
            settings.model

        editor =
            { language = model.editor.language
            , text = model.editor.text
            }
    in
    if enableMonaco then
        Element.column [ width fill, height fill ]
            [ Element.Input.button []
                { onPress = Just (Dispatch "save")
                , label = Element.text "Render"
                }
            , Editor.viewMonacoEditor editor
            ]

    else
        Editor.viewMultilineEditor Save editor

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
    Element.Input.multiline []
        { onChange = onChange
        , text = model.text
        , placeholder = Nothing
        , label = Element.Input.labelHidden "Editor"
        , spellcheck = False
        }
