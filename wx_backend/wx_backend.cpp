#include <wx/wx.h>

void (*func_pointer)();

void print_hello_world() {
    std::cout << "Hello, World! {from func_pointer 2x} xD" << std::endl;
}

extern "C" {
    void set_func_pointer(void (*func)()) {
        func_pointer = func;
    }

    void start_wx_system(int argc, char **argv) {
        std::cout << "Starting wxEntryStart. Where argc: " << argc << ", argv: " << argv << std::endl;

        int n_argc = 3;
        char *n_argv[] = {
            "program_name",
            "argument_1",
            "argument_2"
        };

        
        wxEntryStart(n_argc, n_argv);

        wxTheApp->CallOnInit();
    }

    void update_events_loop() {
        wxTheApp->Yield(true);
    }
}

class CoreFrame : public wxFrame {
public:
    CoreFrame(const wxString& title) : wxFrame(NULL, wxID_ANY, title) {

    }

private:
    void OnExit(wxCommandEvent& event) {
        Close();
    }

    void OnOK(wxCommandEvent& event) {
        // Do something when the button is clicked
    }
};

class MyApp : public wxApp {
public:
    CoreFrame *coreFrame;

    virtual bool OnInit() {
        coreFrame = new CoreFrame("assur_ui App");

        func_pointer();

        coreFrame->Show();

        std::cout << "I am currently on init@|@" << std::endl;
        return true;
    }
};


wxIMPLEMENT_APP_NO_MAIN(MyApp);
