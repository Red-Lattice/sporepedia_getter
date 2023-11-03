/**
* Shifts the camera up and down
* Code originally written by splitwirez, cleaned up by Error
*/
bool Editors_EditorCamera__OnKeyDown::DETOUR(int virtualKey, KeyModifiers modifiers)
{
    bool returnValue = original_function(this, virtualKey, modifiers);
    intrusive_ptr<UTFWin::IWindow> sporepediaRoot = UTFWin::IWindowManager::Get()->GetMainWindow()->FindWindowByID(0x53C6DC80);


    int pCameraDefaultIndex;
    if (App::Property::GetInt32(GetPropertyList(), id("cameraDiDefaultOffsetYIndex"), pCameraDefaultIndex))
    {
        float* pCameraYvalues;
        size_t pSize;
        if (App::Property::GetArrayFloat(GetPropertyList(), id("cameraDiOffsetYs"), pSize, pCameraYvalues))
        {
            bool pCameraUp = (virtualKey == 0x52);

            bool pCameraDown = (virtualKey == 0x46);

            if (!pCameraUp && !pCameraDown) {return returnValue;} // Guard statement
            camChange();

            App::Property* pDefaultIndexProperty;
            GetPropertyList()->GetProperty(id("cameraDiDefaultOffsetYIndex"), pDefaultIndexProperty);
            pDefaultIndexProperty->SetValueInt32(pCameraDefaultIndex);
            returnValue = true;
        }
    }
    return returnValue;
}

/**
* Abstraction method to make code look nicer
**/
void camChange()
{
    if (GetAsyncKeyState(VK_CONTROL)) {return;} // Guard statement

    if (pCameraUp)
    {
        pCameraDefaultIndex = (pCameraDefaultIndex < (pSize - 1)) ? pCameraDefaultIndex++ : 0;
        return;
    }
    pCameraDefaultIndex = (pCameraDefaultIndex > 0) ? pCameraDefaultIndex-- : (pSize - 1);
}