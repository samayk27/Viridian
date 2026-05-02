package vd

import (
	"errors"
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/chzyer/readline"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var cfgFile string

var rootCmd = &cobra.Command{
	Use:   "vd",
	Short: "A command-line agentic coding assistant",
	Long: `Viridian is a command-line agentic coding assistant written in Go.
It provides a powerful interface for developers to interact with their codebase, 
automate tasks, and enhance productivity through intelligent code generation and management.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Welcome to Viridian! Your command-line agentic coding assistant.")
		REPL()
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error executing your CLI '%s'", err)
		os.Exit(1)
	}
}

func REPL() {
	rl, err := readline.NewEx(&readline.Config{
		Prompt:          "vd> ",
		HistoryFile:     os.ExpandEnv("$HOME/.viridian_history"),
		InterruptPrompt: "^C",
		EOFPrompt:       "exit",
	})
	if err != nil {
		fmt.Fprintf(os.Stderr, "REPL init error: %s\n", err)
		return
	}
	defer rl.Close()

	fmt.Println("Entering Viridian REPL mode. Type 'exit' to quit.")
	for {
		line, err := rl.Readline()
		if errors.Is(err, readline.ErrInterrupt) {
			if len(line) == 0 {
				break
			}
			continue
		}
		if errors.Is(err, io.EOF) {
			break
		}
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		if line == "exit" {
			fmt.Println("Exiting Viridian. Goodbye!")
			break
		}

		fmt.Printf("You entered: %s\n", line)
	}
}

func init() {
	cobra.OnInitialize(initConfig)

	//parse CLI args

	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.viridian.yaml)")

	// Cobra also supports local flags, which will only run
	// when this action is called directly.
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

// initConfig reads in config file and ENV variables if set.
func initConfig() {
	if cfgFile != "" {
		// Use config file from the flag.
		viper.SetConfigFile(cfgFile)
	} else {
		// Find home directory.
		home, err := os.UserHomeDir()
		cobra.CheckErr(err)

		// Search config in home directory with name ".viridian" (without extension).
		viper.AddConfigPath(home)
		viper.SetConfigType("yaml")
		viper.SetConfigName(".viridian")
	}

	viper.AutomaticEnv() // read in environment variables that match

	// If a config file is found, read it in.
	if err := viper.ReadInConfig(); err == nil {
		fmt.Fprintln(os.Stderr, "Using config file:", viper.ConfigFileUsed())
	}
}
