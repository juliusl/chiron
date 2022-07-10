port module Main exposing (..)

import Browser
import Element exposing (..)
import Element.Input
import Element.Border as Border
import Element.Font as Font
import Editor exposing (viewMonacoEditor, viewMultilineEditor)
import Html exposing (..)
import Http
import Instructions
import Layout
import Element.Font

type alias Model =
    { editor : Editor
    , instructions : String
    , viewFull : Bool
    , edit : Bool
    }

type alias Editor =
    { text : String, language : String, saved : String }

type alias Command = 
    { onPress : Msg, label: Element Msg }

type Msg
    = ResetText
    | Dispatch String
    | Save String
    | Instructions String
    | ViewFull
    | Edit
    | GotLab (Result Http.Error String)
    | Done

main : Program (Maybe String) Model Msg
main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

-- Init

init : (Maybe String) -> ( Model, Cmd Msg )
init maybelab =
    let
        default = 
            Model { text = "", language = "markdown", saved = "" } "" False False
    in
    case maybelab  of
        Just lab -> 
            (default, getLab lab )
        Nothing -> 
            (default, getLab "" )

-- VIEW

view : Model -> Browser.Document Msg
view model =
    let
        enableFullView =
            model.viewFull
        enableEdit = 
            model.edit
        instructions = 
            model.instructions
        editor = 
            { enableMonaco = False, model = model }
    in
    { title = "Chiron lab portal"
    , body = [
          Layout.view
            { title = ""
            , shrinkContent = enableEdit
            , content = (
                if enableFullView then 
                    Instructions.viewFullPage instructions
                else
                    Instructions.viewInstructions onNext ViewFull Done instructions
            )
            , left_detail = viewCodeEditor editor
            , right_detail = viewCommands [ { onPress = Edit, label = (Element.text "Edit")} ]
            }
        ]
    }

viewCommands : (List Command) -> Element Msg 
viewCommands =
    (\commands ->
                Element.column 
            [ Border.widthEach { top = 0, right = 0, bottom = 0, left = 1 }
            , paddingEach { top = 4, right = 8, left = 10, bottom = 4}
            , Border.color (Element.rgb255 145 145 145)
            ] (
                commands |> List.map
                    (\command -> 
                    Element.Input.button 
                    [ Element.Font.size 14
                    , Element.Font.family [  Font.typeface "system-ui" ]
                    ] { onPress = Just command.onPress, label = command.label }
                    )
            )
    )


viewCodeEditor : { enableMonaco : Bool, model : Model } -> Element Msg
viewCodeEditor settings =
    let
        visible =
            settings.model.edit

        enableMonaco =
            settings.enableMonaco

        model =
            settings.model

        editor =
            { language = model.editor.language
            , text = model.editor.text
            }
    in
    if visible then 
            if enableMonaco then
                Element.column [ width fill, height fill ]
                [   Element.Input.button []
                    { onPress = Just (Dispatch "save")
                    , label = Element.text "Render"
                    }
                ,viewMonacoEditor editor
                ]

        else
        viewMultilineEditor Save editor
    else
        Element.text ""

onNext : List String -> Maybe Msg
onNext remaining =
    if List.isEmpty remaining then
        Nothing
    else
        Just (Instructions (String.join "\n" remaining))

-- UPDATE

update : Msg -> Model -> ( Model, Cmd msg )
update msg model =
    let
        editor =
            model.editor
    in
    case msg of
        ResetText ->
            ( { model | editor = { editor | text = editor.saved } }, Cmd.none )

        Save content ->
            ( { model | editor = { editor | text = content, saved = content }, instructions = content }, Cmd.none )

        Dispatch cmd ->
            ( model, dispatchEditorCmd cmd )

        Instructions instructions ->
            ( { model | instructions = instructions }, Cmd.none )

        ViewFull -> 
            ( { model | editor = { editor | text = editor.saved }, instructions = model.editor.text, viewFull = True }, Cmd.none )

        Edit ->
            ( { model | edit = (not model.edit) }, Cmd.none )

        Done ->
            ( { model | instructions = model.editor.text }, Cmd.none )
        
        GotLab result -> 
            case result of 
                Ok lab -> 
                    ({ model | editor = { editor | text = lab, saved = lab }, instructions = lab }, Cmd.none )
                Err _ -> 
                    (model, Cmd.none)


-- SUBSCRIPTIONS
-- If Monaco is enabled, this will allow us to pipe commands to the editor
port dispatchEditorCmd : String -> Cmd msg

-- This is called by monaco to pass the current value of it's editor
port saveContent : (String -> msg) -> Sub msg


subscriptions : Model -> Sub Msg
subscriptions _ =
    saveContent Save

-- API

getLab : String -> Cmd Msg
getLab lab = 
    Http.get 
    { url = String.concat [ "/lab/", lab ]
    , expect = Http.expectString GotLab
    }